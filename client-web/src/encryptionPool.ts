import { Keys } from "../bridge/pkg/qqself_client_web_bridge"
import { InputType, OutputType } from "./encryptionPool.worker"
import { log } from "./logger"

// Encrypting, decrypting and generating keys are very CPU intensive operations
// and in the world of JavaScript may block event loop for the very long time.
// To avoid it we run multiple Worker processes that handles those operation in
// background, kinda like a dedicated ThreadPool
export class EncryptionPool {
  private workers: Worker[] = []
  private initDone = false

  constructor() {
    const count = navigator.hardwareConcurrency
    for (let i = 0; i < count; i++) {
      // URL is needed so that Vite will recognize it and compile as a separate file
      const worker = new Worker(new URL("./encryptionPool.worker.ts", import.meta.url), {
        type: "module",
      })
      this.workers.push(worker)
    }
    log(`EncryptionPool started ${count} workers`)
  }

  // To avoid sending keys with every payload we send it once to each worker with the init message
  async ensureInitialized(keys: Keys): Promise<void> {
    if (this.initDone) return Promise.resolve()

    const value = keys.serialize()
    const results: Promise<void>[] = []
    for (const worker of this.workers) {
      results.push(
        new Promise((resolve, reject) => {
          this.sendMessage(worker, { kind: "Init", keys: value })
          worker.onmessage = (event: any) => {
            const result: OutputType = event.data
            if (result.kind == "Initialized") {
              resolve()
            } else {
              reject(new Error("Worker failed to initialize: " + result))
            }
          }
        })
      )
    }
    await Promise.all(results)
    this.initDone = true
  }

  private sendMessage(worker: Worker, input: InputType) {
    worker.postMessage(input)
  }

  private getWorker(): Worker {
    const workers = this.workers
    return workers[Math.floor(Math.random() * workers.length)]
  }

  private sendPayload(
    kind: "Encrypt" | "Decrypt",
    value: string,
    callback: (result: Error | string) => void
  ) {
    const worker = this.getWorker()
    this.sendMessage(worker, { kind, value })
    worker.onmessage = (event: any) => {
      const result: OutputType = event.data
      if (result.kind == "Error") {
        callback(result.error)
      } else if (result.kind == "Plaintext") {
        callback(result.plaintext)
      } else {
        callback(new Error("Unexpected result from worker: " + JSON.stringify(result)))
      }
    }
  }

  // Generate new encryption keys. Rejects in case of worker errors
  async generateNewKeys(): Promise<Keys> {
    const worker = this.getWorker()
    this.sendMessage(worker, { kind: "GenerateKeys" })
    return new Promise((resolve, reject) => {
      worker.onmessage = (event: any) => {
        const result: OutputType = event.data
        if (result.kind == "Error") {
          reject(result.error)
        } else if (result.kind == "Keys") {
          const keys = Keys.deserialize(result.keys)
          resolve(keys)
        } else {
          reject(new Error("Unexpected result from worker: " + result))
        }
      }
    })
  }

  // Queue message for decryption. Once result is available the provided callback will be called
  // Callback based API is more convenient for use cases when we have many messages to decrypt
  async queueForDecryption(msg: string, keys: Keys, callback: (result: Error | string) => void) {
    await this.ensureInitialized(keys)
    this.sendPayload("Decrypt", msg, callback)
  }

  async decryptAll(msgs: string[], keys: Keys): Promise<string[]> {
    if (!msgs.length) {
      return Promise.resolve([])
    }
    await this.ensureInitialized(keys)
    const finished: string[] = []
    return new Promise((resolve, reject) => {
      const cb = (result: Error | string) => {
        if (typeof result == "string") {
          finished.push(result)
          if (finished.length == msgs.length) {
            resolve(finished)
          }
        } else {
          reject(result)
        }
      }
      for (const msg of msgs) {
        this.queueForDecryption(msg, keys, cb)
      }
    })
  }

  // Queue message for encryption. Once result is available the provided callback will be called
  // Callback based API is more convenient for use cases when we have many messages to decrypt
  async queueForEncryption(msg: string, keys: Keys, callback: (result: Error | string) => void) {
    await this.ensureInitialized(keys)
    this.sendPayload("Encrypt", msg, callback)
  }
}
