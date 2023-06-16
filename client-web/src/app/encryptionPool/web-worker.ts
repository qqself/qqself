import init, { Keys, initialize } from "../../../bridge/pkg/qqself_client_web_bridge"
import { InputType, OutputType, processMessage } from "./worker"

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

let initCompleted: Promise<void> | null = null

self.onmessage = async (event: MessageEvent<InputType>) => {
  const input = event.data
  if (input.kind == "Init") {
    initCompleted = new Promise<void>(async (resolve, reject) => {
      await init()
      initialize()
      id = input.workerId
      if (input.keys) {
        cachedKeys = Keys.deserialize(input.keys)
      }
      send({ kind: "Initialized" }, input.taskId)
      resolve()
    })
  } else if (initCompleted) {
    // We need to initialize WebAssembly before we would be able to do anything
    // ensure here that it's completed
    initCompleted.then(() => processMessage(event.data, cachedKeys, send))
  } else {
    throw new Error("Init call missing")
  }
}
