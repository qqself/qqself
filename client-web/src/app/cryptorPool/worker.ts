import { Cryptor } from "../../../qqself_core"
import { EncryptedEntry } from "../api"
import { DecryptedEntry } from "./pool"

export type SignInput = { kind: "find"; lastSyncId: string | null } | { kind: "delete" }
export type InputType =
  | { kind: "Init"; taskId: number; workerId: number; keys: string | null }
  | { kind: "Encrypt"; taskId: number; text: string }
  | { kind: "Decrypt"; taskId: number; payload: EncryptedEntry }
  | { kind: "GenerateCryptor"; taskId: number }
  | { kind: "Sign"; taskId: number; data: SignInput }

export type OutputType =
  | { kind: "Initialized" }
  | { kind: "Error"; error: Error }
  | { kind: "Cryptor"; keys: string }
  | { kind: "Decrypted"; decrypted: DecryptedEntry }
  | { kind: "Encrypted"; encrypted: EncryptedPayload }
  | { kind: "Signed"; payload: string }

export type EncryptedPayload = Pick<EncryptedEntry, "payload">

export interface WorkerResult {
  workerId: number
  taskId: number
  output: OutputType
}

const generateKeys = () => {
  return Cryptor.generate_new().serialize_keys()
}

const decrypt = (entry: EncryptedEntry, cryptor: Cryptor | null): DecryptedEntry => {
  if (!cryptor) throw new Error("Worker has to be initialized first")
  const plaintext = cryptor.decrypt(entry.payload)
  return { id: entry.id, text: plaintext }
}

const encrypt = (text: string, cryptor: Cryptor | null): EncryptedPayload => {
  if (!cryptor) throw new Error("Worker has to be initialized first")
  const payload = cryptor.encrypt(text)
  return { payload }
}

const sign = (cryptor: Cryptor | null, data: SignInput): string => {
  if (!cryptor) throw new Error("Worker has to be initialized first")
  if (data.kind == "delete") {
    return cryptor.sign_delete_token()
  } else {
    return cryptor.sign_find_token(data.lastSyncId ?? undefined)
  }
}

export const processMessage = (
  input: InputType,
  cryptor: Cryptor | null,
  callback: (result: OutputType, taskId: number) => void,
) => {
  switch (input.kind) {
    case "GenerateCryptor":
      callback({ kind: "Cryptor", keys: generateKeys() }, input.taskId)
      break
    case "Decrypt":
      try {
        const decrypted = decrypt(input.payload, cryptor)
        callback({ kind: "Decrypted", decrypted }, input.taskId)
      } catch (error) {
        callback({ kind: "Error", error: error as Error }, input.taskId)
      }
      break
    case "Encrypt":
      try {
        const encrypted = encrypt(input.text, cryptor)
        callback({ kind: "Encrypted", encrypted }, input.taskId)
      } catch (error) {
        callback({ kind: "Error", error: error as Error }, input.taskId)
      }
      break
    case "Sign":
      try {
        const payload = sign(cryptor, input.data)
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
