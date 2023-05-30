// Worker for running encryption related login outside of the main thread to avoid blocking main event loop
import init, { Keys } from "../bridge/pkg/qqself_client_web_bridge"

export type InputType =
  | { kind: "Init"; keys: string }
  | { kind: "Encrypt"; value: string }
  | { kind: "Decrypt"; value: string }
  | { kind: "GenerateKeys" }

export type OutputType =
  | { kind: "Initialized" }
  | { kind: "Error"; error: Error }
  | { kind: "Keys"; keys: string }
  | { kind: "Plaintext"; plaintext: string }
  | { kind: "Payload"; payload: string }

let webAssemblyReady = false
let initKey: Keys | null = null

const callback = (result: OutputType) => {
  postMessage(result)
}

self.onmessage = async (event: any) => {
  if (!webAssemblyReady) {
    await init()
    webAssemblyReady = true
  }
  const input: InputType = event.data
  switch (input.kind) {
    case "Init": // Receive the key and cache it for all following requests
      initKey = Keys.deserialize(input.keys)
      callback({ kind: "Initialized" })
      break
    case "GenerateKeys": // Generate new keys, as it's a shallow pointer serialize it before sending
      const keys = Keys.createNewKeys()
      callback({ kind: "Keys", keys: keys.serialize() })
      break
    case "Decrypt": // Decrypt the payload if init was done, error otherwise
      if (initKey) {
        const plaintext = initKey.decrypt(input.value)
        callback({ kind: "Plaintext", plaintext })
      } else {
        callback({
          kind: "Error",
          error: new Error("Worker has to be initialized first"),
        })
      }
      break
    default: // TODO Encrypt is missing
      callback({
        kind: "Error",
        error: new Error(`Operation ${input.kind} is not supported yet`),
      })
      break
  }
}
