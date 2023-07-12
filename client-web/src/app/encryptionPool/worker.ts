import { Keys } from "../../../bridge/pkg/qqself_client_web_bridge"
import { EncryptedEntry } from "../api"
import { DecryptedEntry } from "./pool"

export type SignInput = { kind: "find"; lastSyncId: string | null } | { kind: "delete" }
export type InputType =
  | { kind: "Init"; taskId: string; workerId: string; keys: string | null }
  | { kind: "Encrypt"; taskId: string; text: string }
  | { kind: "Decrypt"; taskId: string; payload: EncryptedEntry }
  | { kind: "GenerateKeys"; taskId: string }
  | { kind: "Sign"; taskId: string; data: SignInput }

export type OutputType =
  | { kind: "Initialized" }
  | { kind: "Error"; error: Error }
  | { kind: "Keys"; keys: string }
  | { kind: "Decrypted"; decrypted: DecryptedEntry }
  | { kind: "Encrypted"; encrypted: EncryptedPayload }
  | { kind: "Signed"; payload: string }

export type EncryptedPayload = Pick<EncryptedEntry, "payload">

export interface WorkerResult {
  workerId: string
  taskId: string
  output: OutputType
}

const generateKeys = () => {
  return Keys.createNewKeys().serialize()
}

const decrypt = (entry: EncryptedEntry, keys: Keys | null): DecryptedEntry => {
  if (!keys) throw new Error("Worker has to be initialized first")
  const plaintext = keys.decrypt(entry.payload)
  return { id: entry.id, text: plaintext }
}

const encrypt = (text: string, keys: Keys | null): EncryptedPayload => {
  if (!keys) throw new Error("Worker has to be initialized first")
  const payload = keys.encrypt(text)
  return { payload }
}

const sign = (keys: Keys | null, data: SignInput): string => {
  if (!keys) throw new Error("Worker has to be initialized first")
  if (data.kind == "delete") {
    return keys.sign_delete_token()
  } else {
    return keys.sign_find_token(data.lastSyncId ?? undefined)
  }
}

export const processMessage = (
  input: InputType,
  keys: Keys | null,
  callback: (result: OutputType, taskId: string) => void,
) => {
  switch (input.kind) {
    case "GenerateKeys":
      callback({ kind: "Keys", keys: generateKeys() }, input.taskId)
      break
    case "Decrypt":
      try {
        const decrypted = decrypt(input.payload, keys)
        callback({ kind: "Decrypted", decrypted }, input.taskId)
      } catch (error) {
        callback({ kind: "Error", error: error as Error }, input.taskId)
      }
      break
    case "Encrypt":
      try {
        const encrypted = encrypt(input.text, keys)
        callback({ kind: "Encrypted", encrypted }, input.taskId)
      } catch (error) {
        callback({ kind: "Error", error: error as Error }, input.taskId)
      }
      break
    case "Sign":
      try {
        const payload = sign(keys, input.data)
        callback({ kind: "Signed", payload }, input.taskId)
      } catch (error) {
        callback({ kind: "Error", error: error as Error }, input.taskId)
      }
      break
    default:
      callback(
        {
          kind: "Error",
          error: new Error(`Bad input: ${JSON.stringify(input)}`),
        },
        input.taskId,
      )
      break
  }
}
