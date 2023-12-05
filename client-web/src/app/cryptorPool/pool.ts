import { Cryptor } from "../../../qqself_core/qqself_core"
import { info } from "../../logger"
import { isBrowser } from "../../utils"
import { EncryptedEntry } from "../api"
import { ThreadWorker } from "./thread-worker"
import { InputType, SignInput, WorkerResult } from "./worker"

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
  onCompleted: (value: never) => void
  onError: (value: never) => void
}

interface PoolWorker {
  worker: WorkerType
  status: "free" | "busy"
}

interface CryptorGeneratorPool {
  generateCryptor(): Promise<Cryptor>
}

// Encrypting, decrypting and generating keys are very CPU intensive operations
// and in the world of JavaScript may block event loop for the very long time.
// To avoid it we run multiple Worker processes that handles those operation in
// background, kinda like a dedicated ThreadPool
export class CryptorPool {
  private workers = new Map<number, PoolWorker>()
  private tasks = new Map<number, PoolTask>()
  private lastTaskId = 0 // Counter to assign each task a unique identifier

  private constructor(cryptor: Cryptor | null) {
    // CryptorGeneratorPool can only generate keys and 1 worker should be enough
    const workersCount = cryptor ? getCpuCount() : 1
    const serializedKeys = cryptor ? cryptor.serialize_keys() : null
    for (let i = 0; i < workersCount; i++) {
      const worker = isBrowser
        ? new Worker(new URL("./web-worker.ts", import.meta.url), { type: "module" })
        : new ThreadWorker()
      worker.addEventListener("message", this.onWorkerDone.bind(this))
      this.workers.set(i, { worker, status: "free" })
      worker.postMessage({
        kind: "Init",
        workerId: i,
        taskId: this.lastTaskId++,
        keys: serializedKeys,
      })
    }
    info(`EncryptionPool started ${workersCount} workers`)
  }

  static initWithCryptor(cryptor: Cryptor) {
    return new CryptorPool(cryptor)
  }

  static initCryptorGenerator(): CryptorGeneratorPool {
    return new CryptorPool(null)
  }

  workersCount() {
    return Object.keys(this.workers).length
  }

  private onWorkerDone(event: Event) {
    const data = (event as MessageEvent<WorkerResult>).data
    if (data.output.kind == "Initialized") {
      // Worker initialized
    } else {
      const task = this.tasks.get(data.taskId)
      const worker = this.workers.get(data.workerId)
      if (!task || !worker) throw new Error(`Task or Worker cannot be found in a pool`)
      if (data.output.kind == "Error") {
        task.onError(data.output.error as never)
      } else {
        task.onCompleted(data.output as never)
      }
      this.tasks.delete(data.taskId)
      worker.status = "free"
      this.allocateWork()
    }
  }

  allocateWork() {
    for (;;) {
      const nextTask = Array.from(this.tasks).find((v) => v[1].status == "pending")
      if (!nextTask) {
        return // No tasks available
      }
      const nextWorker = Array.from(this.workers).find((v) => v[1].status == "free")
      if (!nextWorker) {
        return // No worker available
      }
      // Worker and Task found, update it's status and start a task
      nextTask[1].status = "progress"
      nextWorker[1].status = "busy"
      nextWorker[1].worker.postMessage(nextTask[1].input)
    }
  }

  async generateCryptor(): Promise<Cryptor> {
    const task = new Promise((resolve, reject) => {
      const taskId = this.lastTaskId++
      const task: PoolTask = {
        input: { kind: "GenerateCryptor", taskId },
        status: "pending",
        onCompleted: (output: { keys: string }) => {
          resolve(Cryptor.from_deserialized_keys(output.keys))
        },
        onError: reject,
      }
      this.tasks.set(taskId, task)
      this.allocateWork()
    })
    return task as Promise<Cryptor>
  }

  async encrypt(text: string): Promise<EncryptedPayload> {
    const task = new Promise((resolve, reject) => {
      const taskId = this.lastTaskId++
      const task: PoolTask = {
        input: { kind: "Encrypt", taskId, text },
        status: "pending",
        onCompleted: (output: { encrypted: EncryptedPayload }) => {
          resolve(output.encrypted)
        },
        onError: reject,
      }
      this.tasks.set(taskId, task)
      this.allocateWork()
    })
    return task as Promise<EncryptedPayload>
  }

  async sign(data: SignInput): Promise<string> {
    const task = new Promise((resolve, reject) => {
      const taskId = this.lastTaskId++
      const task: PoolTask = {
        input: { kind: "Sign", taskId, data },
        status: "pending",
        onCompleted: (output: { payload: string }) => {
          resolve(output.payload)
        },
        onError: reject,
      }
      this.tasks.set(taskId, task)
      this.allocateWork()
    })
    return task as Promise<string>
  }

  async decrypt(payload: EncryptedEntry): Promise<DecryptedEntry> {
    const task = new Promise((resolve, reject) => {
      const taskId = this.lastTaskId++
      const task: PoolTask = {
        input: { kind: "Decrypt", taskId, payload },
        status: "pending",
        onCompleted: (output: { decrypted: DecryptedEntry }) => {
          resolve(output.decrypted)
        },
        onError: reject,
      }
      this.tasks.set(taskId, task)
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

  const cryptor = Cryptor.generate_new()
  const pool = CryptorPool.initWithCryptor(cryptor)

  describe("CryptorPool", () => {
    test("generateCryptor", async () => {
      const cryptor = await pool.generateCryptor()
      const keysHash = cryptor.public_key_hash()
      expect(keysHash.length).toBeTruthy()
    })

    test("CryptorGeneratorPool", async () => {
      const pool = CryptorPool.initCryptorGenerator()
      const cryptor = await pool.generateCryptor()
      const keysHash = cryptor.public_key_hash()
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
      const cryptor1 = await pool.generateCryptor()
      const cryptor2 = await pool.generateCryptor()
      const pool1 = CryptorPool.initWithCryptor(cryptor1)
      const pool2 = CryptorPool.initWithCryptor(cryptor2)
      const payload = await pool1.encrypt("foo")
      return expect(pool2.decrypt({ payload: payload.payload, id: "1" })).rejects.toBe(
        "Failed to decrypt AES key",
      )
    })
  })
}
