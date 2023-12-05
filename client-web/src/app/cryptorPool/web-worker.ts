import init, { Cryptor, initialize } from "../../../qqself_core"
import { InputType, OutputType, processMessage } from "./worker"

let cryptor: Cryptor | null = null
let id: number | null = null

const send = (output: OutputType, taskId: number) => {
  if (id == null) {
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

self.onmessage = (event: MessageEvent<InputType>) => {
  const input = event.data
  if (input.kind == "Init") {
    initCompleted = new Promise<void>((resolve) => {
      // HACK Nested `then` is ugly, but we need to ensure that `initCompleted` becomes a Promise
      //      right away, so that other concurrent calls will end up waiting for it to complete
      void init().then(() => {
        initialize()
        id = input.workerId
        if (input.keys) {
          cryptor = Cryptor.from_deserialized_keys(input.keys)
        }
        send({ kind: "Initialized" }, input.taskId)
        resolve()
      })
    })
  } else if (initCompleted) {
    // We need to initialize WebAssembly before we would be able to do anything
    // ensure here that it's completed
    void initCompleted.then(() => processMessage(event.data, cryptor, send))
  } else {
    throw new Error("Init call missing")
  }
}
