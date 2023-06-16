import { Keys } from "../../../bridge/pkg/qqself_client_web_bridge"
import { EncryptedEntry } from "../api"
import { InputType, WorkerResult } from "./worker"
import { info } from "../../logger"
import { ThreadWorker } from "./thread-worker"
import { isBrowser } from "../../utils"

export interface DecryptedEntry {
  id: string
  text: string
}

export type EncryptedPayload = Pick<EncryptedEntry, "payload">
type WorkerType = Worker | ThreadWorker

const getCpuCount = () => {
  if (isBrowser) {
    return navigator.hardwareConcurrency
  } else {
    // Right way to use require("os").cpus().length but we don't use thread-workers yet
    return 2
  }
}

interface PoolTask {
  input: InputType
  status: "pending" | "progress"
  onCompleted: (value: any) => void
  onError: (value: any) => void
}

interface PoolWorker {
  worker: WorkerType
  status: "free" | "busy"
}

interface KeylessEncryptionPool {
  generateNewKeys(): Promise<Keys>
}

// Encrypting, decrypting and generating keys are very CPU intensive operations
// and in the world of JavaScript may block event loop for the very long time.
// To avoid it we run multiple Worker processes that handles those operation in
// background, kinda like a dedicated ThreadPool
export class EncryptionPool {
  private workers: Record<string, PoolWorker> = {}
  private tasks: Record<string, PoolTask> = {}
  private lastTaskId = 0 // Counter to assign each task a unique identifier

  private constructor(keys: Keys | null) {
    // Keyless pool can only generate keys and 1 worker should be enough
    const workersCount = keys ? getCpuCount() : 1
    const serializedKeys = keys ? keys.serialize() : null
    for (let i = 0; i < workersCount; i++) {
      const worker = isBrowser
        ? new Worker(new URL("./web-worker.ts", import.meta.url), { type: "module" })
        : new ThreadWorker()
      worker.addEventListener("message", this.onWorkerDone.bind(this))
      this.workers[i] = { worker, status: "free" }
      worker.postMessage({
        kind: "Init",
        workerId: String(i),
        taskId: String(this.lastTaskId++),
        keys: serializedKeys,
      })
    }
    info(`EncryptionPool started ${workersCount} workers`)
  }

  static initWithKeys(keys: Keys) {
    return new EncryptionPool(keys)
  }

  static initKeyless(): KeylessEncryptionPool {
    return new EncryptionPool(null)
  }

  workersCount() {
    return Object.keys(this.workers).length
  }

  private onWorkerDone(event: Event) {
    const data = (event as MessageEvent<WorkerResult>).data
    if (data.output.kind == "Initialized") {
      // Worker initialized
    } else {
      const task = this.tasks[data.taskId]
      if (data.output.kind == "Error") {
        task.onError(data.output.error)
      } else {
        task.onCompleted(data.output)
      }
      delete this.tasks[data.taskId]
      this.workers[data.workerId].status = "free"
      this.allocateWork()
    }
  }

  allocateWork() {
    for (;;) {
      const nextTask = Object.entries(this.tasks).find((v) => v[1].status == "pending")
      if (!nextTask) {
        return // No tasks available
      }
      const nextWorker = Object.entries(this.workers).find((v) => v[1].status == "free")
      if (!nextWorker) {
        return // No worker available
      }
      // Worker and Task found, update it's status and start a task
      this.tasks[nextTask[0]].status = "progress"
      this.workers[nextWorker[0]].status = "busy"
      nextWorker[1].worker.postMessage(nextTask[1].input)
    }
  }

  async generateNewKeys(): Promise<Keys> {
    const task = new Promise((resolve, reject) => {
      const taskId = String(this.lastTaskId++)
      const task: PoolTask = {
        input: { kind: "GenerateKeys", taskId },
        status: "pending",
        onCompleted: (output: { keys: string }) => {
          resolve(Keys.deserialize(output.keys))
        },
        onError: reject,
      }
      this.tasks[taskId] = task
      this.allocateWork()
    })
    return task as Promise<Keys>
  }

  async encrypt(text: string): Promise<EncryptedPayload> {
    const task = new Promise((resolve, reject) => {
      const taskId = String(this.lastTaskId++)
      const task: PoolTask = {
        input: { kind: "Encrypt", taskId, text },
        status: "pending",
        onCompleted: (output: { encrypted: EncryptedPayload }) => {
          resolve(output.encrypted)
        },
        onError: reject,
      }
      this.tasks[taskId] = task
      this.allocateWork()
    })
    return task as Promise<EncryptedPayload>
  }

  async decrypt(payload: EncryptedEntry): Promise<DecryptedEntry> {
    const task = new Promise((resolve, reject) => {
      const taskId = String(this.lastTaskId++)
      const task: PoolTask = {
        input: { kind: "Decrypt", taskId, payload },
        status: "pending",
        onCompleted: (output: { decrypted: DecryptedEntry }) => {
          resolve(output.decrypted)
        },
        onError: reject,
      }
      this.tasks[taskId] = task
      this.allocateWork()
    })
    return task as Promise<DecryptedEntry>
  }

  async decryptAll(msgs: EncryptedEntry[]): Promise<DecryptedEntry[]> {
    return Promise.all(msgs.map((msg) => this.decrypt(msg)))
  }
}

if (import.meta.vitest) {
  const { describe, test, expect } = import.meta.vitest

  const keys = Keys.createNewKeys()
  const pool = EncryptionPool.initWithKeys(keys)

  describe("encryptionPool", () => {
    test("generateKeys", async () => {
      const keys = await pool.generateNewKeys()
      const keysHash = keys.public_key_hash()
      expect(keysHash.length).toBeTruthy()
    })

    test("generateKeys keyless pool", async () => {
      const pool = EncryptionPool.initKeyless()
      const keys = await pool.generateNewKeys()
      const keysHash = keys.public_key_hash()
      expect(keysHash.length).toBeTruthy()
    })

    test("encryption/decryption", async () => {
      const input = "foo"
      const encrypted = await pool.encrypt(input)
      expect(encrypted.payload).toBeTruthy()
      const decrypted = await pool.decrypt({ ...encrypted, id: "id" })
      expect(input).toEqual(decrypted.text)
    })

    test("decrypt all", async () => {
      const payload1 = await pool.encrypt("msg1")
      const payload2 = await pool.encrypt("msg2")
      const decrypted = await pool.decryptAll([
        { ...payload1, id: "id1" },
        { ...payload2, id: "id2" },
      ])
      expect(decrypted).toEqual([
        { id: "id1", text: "msg1" },
        { id: "id2", text: "msg2" },
      ])
    })

    test("decrypt many", async () => {
      const msgCount = pool.workersCount() * 5
      const msgs = Array(msgCount)
        .fill(0)
        .map((_, i) => `msg-${i}`)
      const encrypted = await Promise.all(msgs.map((v) => pool.encrypt(v)))
      const decrypted = await pool.decryptAll(encrypted.map((v) => ({ ...v, id: "id" })))
      expect(decrypted).toEqual(msgs.map((v) => ({ id: "id", text: v })))
    })

    test("error handling", async () => {
      const keys1 = await pool.generateNewKeys()
      const keys2 = await pool.generateNewKeys()
      const pool1 = EncryptionPool.initWithKeys(keys1)
      const pool2 = EncryptionPool.initWithKeys(keys2)
      const payload = await pool1.encrypt("foo")
      return expect(pool2.decrypt({ payload: payload.payload, id: "1" })).rejects.toBe(
        "Failed to decrypt AES key"
      )
    })
  })
}
