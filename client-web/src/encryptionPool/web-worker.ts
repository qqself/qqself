import init, { Keys, initialize } from "../../bridge/pkg"
import { InputType, OutputType, WorkerResult, processMessage } from "./worker"

let cachedKeys: Keys | null = null
let id: string | null = null

const send = (output: OutputType, taskId: string) => {
  if (!id) {
    postMessage({
      output: {
        kind: "Error",
        error: new Error(`Worker id is not set, no Init message were received`),
      },
    })
    return
  }
  postMessage({ workerId: id, taskId, output })
}

self.onmessage = async (event: MessageEvent<InputType>) => {
  const input = event.data
  if (input.kind == "Init") {
    await init()
    initialize()
    id = input.workerId
    if (input.keys) {
      cachedKeys = Keys.deserialize(input.keys)
    }
    send({ kind: "Initialized" }, input.taskId)
    return
  }

  processMessage(event.data, cachedKeys, send)
}
