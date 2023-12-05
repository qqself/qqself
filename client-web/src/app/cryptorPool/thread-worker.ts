import { Cryptor } from "../../../qqself_core"
import { InputType, OutputType, processMessage, WorkerResult } from "./worker"

// Right way is to go with node:worker_threads, but unfortunately
// building it is challenging in Vite: https://github.com/vitejs/vite/pull/3932
// For now create a dummy worker with similar API to Web Workers
export class ThreadWorker extends EventTarget {
  cryptor: Cryptor | null = null
  id: number | null = null

  postMessage(input: InputType) {
    if (input.kind == "Init") {
      this.id = input.workerId
      if (input.keys) {
        this.cryptor = Cryptor.from_deserialized_keys(input.keys)
      }
      this.send({ kind: "Initialized" }, input.taskId)
      return
    }

    return processMessage(input, this.cryptor, this.send.bind(this))
  }

  send(output: OutputType, taskId: number) {
    if (this.id == null) {
      this.dispatchEvent(
        new MessageEvent("message", {
          data: {
            output: {
              kind: "Error",
              error: new Error(`Worker id is not set, no Init message were received`),
            },
          },
        }),
      )
      return
    }
    const data: WorkerResult = { workerId: this.id, taskId, output }
    this.dispatchEvent(new MessageEvent("message", { data }))
  }
}
