import { API, Keys, Request } from "../bridge/pkg/qqself_client_web_bridge"

type ApiError = {
  timestamp: number
  error_code: string
  error: string
}

const http = async (req: Request): Promise<Response> => {
  const url = req.url.replace("https://api.qqself.com", import.meta.env.VITE_API_HOST)
  const resp = await fetch(url, {
    method: "POST",
    body: req.payload,
    headers: {
      "Content-Type": req.contentType,
    },
  })
  if (resp.status != 200) {
    const err: ApiError = await resp.json()
    throw new Error("API error: " + err.error)
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

// Call Delete sync API endpoint
export const deleteAccount = async (keys: Keys): Promise<void> => {
  await http(API.createApiDeleteRequest(keys))
}

if (import.meta.vitest) {
  const { describe, test, expect } = import.meta.vitest

  describe("API", () => {
    test("Create new keys", async () => {
      const keys = Keys.createNewKeys()
      expect(keys).toBeTruthy()
    })

    test("API", async () => {
      // First find call no data
      const keys = Keys.createNewKeys()
      const lines = await find(keys)
      expect(lines).toEqual([])

      // Add couple of messages
      await set(keys, "msg1")
      await set(keys, "msg2")

      // Get those back
      const got = await find(keys)
      const plaintext = got.map((v) => keys.decrypt(v.split(":")[1]))
      expect(plaintext.sort()).toEqual(["msg1", "msg2"]) // Sort order of items with the same timestamp is not defined

      // Delete it all
      await deleteAccount(keys)
    })
  })
}
