import { API, Keys, Request } from "../core/pkg/qqself_client_web_core"
import { EncryptionPool } from "./encryptionPool"

type ApiError = {
  timestamp: number
  error_code: string
  error: string
}

const http = async (req: Request): Promise<Response> => {
  const resp = await fetch(req.url, {
    method: "POST",
    body: req.payload,
    headers: {
      "Content-Type": req.contentType,
    },
  })
  if (resp.status != 200) {
    const err: ApiError = await resp.json()
    throw new Error("API find error: " + err.error)
  }
  return resp
}

// Call Set sync API endpoint
export const set = async (keys: Keys, msg: string): Promise<void> => {
  await http(API.createApiSetRequest(keys, msg))
}

// Call Find sync API endpoint
export const find = async (keys: Keys): Promise<string[]> => {
  const resp = await http(API.createApiFindRequest(keys))
  if (!resp.body) {
    throw new Error("API find error: no body")
  }
  const lines = await resp.text()
  if (!lines) {
    return [] // Find returned no lines
  }
  return lines.split("\n").filter((v) => v) // Filter out empty line
}
