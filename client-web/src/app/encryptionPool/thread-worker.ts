import { Keys } from "../../../bridge/pkg/qqself_client_web_bridge"
import { InputType, OutputType, WorkerResult, processMessage } from "./worker"

// Right way is to go with node:worker_threads, but unfortunately
// building it is challenging in Vite: https://github.com/vitejs/vite/pull/3932
// For now create a dummy worker with similar API to Web Workers
export class ThreadWorker extends EventTarget {
  cachedKeys: Keys | null = null
  id: string | null = null

  postMessage(input: InputType) {
    if (input.kind == "Init") {
      this.id = input.workerId
      if (input.keys) {
        this.cachedKeys = Keys.deserialize(input.keys)
      }
      this.send({ kind: "Initialized" }, input.taskId)
      return
    }

    processMessage(input, this.cachedKeys, this.send.bind(this))
  }

  send(output: OutputType, taskId: string) {
    if (!this.id) {
      this.dispatchEvent(
        new MessageEvent("message", {
          data: {
            output: {
              kind: "Error",
              error: new Error(`Worker id is not set, no Init message were received`),
            },
          },
        })
      )
      return
    }
    const data: WorkerResult = { workerId: this.id, taskId, output }
    this.dispatchEvent(new MessageEvent("message", { data }))
  }
}
