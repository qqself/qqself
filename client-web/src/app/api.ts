import { API, Keys, Request } from "../../bridge/pkg/qqself_client_web_bridge"

interface ApiError {
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
    const err = (await resp.json()) as ApiError
    throw new Error("API error: " + err.error)
  }
  return resp
}

// Call Set sync API endpoint
export const set = async (keys: Keys, msg: string): Promise<string> => {
  const resp = await http(API.createApiSetRequest(keys, msg))
  return resp.text()
}

export interface EncryptedEntry {
  id: string
  payload: string
}

// Call Find sync API endpoint
export const find = async (keys: Keys, lastId: string | null): Promise<EncryptedEntry[]> => {
  const resp = await http(API.createApiFindRequest(keys, lastId || undefined))
  if (!resp.body) {
    throw new Error("API find error: no body")
  }
  const lines = await resp.text()
  if (!lines) {
    return [] // Find returned no lines
  }
  return lines
    .split("\n")
    .filter((v) => v) // Filter out empty lines
    .map((v) => {
      // TODO This is ugly manual id parsing, parse it properly via PayloadId::parse
      const entry = v.split(":")
      return { id: entry[0], payload: entry[1] }
    })
}

// Call Delete sync API endpoint
export const deleteAccount = async (keys: Keys): Promise<void> => {
  await http(API.createApiDeleteRequest(keys))
}

if (import.meta.vitest) {
  const { describe, test, expect } = import.meta.vitest

  const wait = (seconds: number) => new Promise((resolve) => setTimeout(resolve, seconds * 1000))

  describe("API", () => {
    test("Create new keys", () => {
      const keys = Keys.createNewKeys()
      expect(keys).toBeTruthy()
    })

    test("API", async () => {
      // First find call no data
      const keys = Keys.createNewKeys()
      const lines = await find(keys, null)
      expect(lines).toEqual([])

      // Add couple of messages
      await set(keys, "msg1")
      await set(keys, "msg2")

      // Get all messages back
      const got = await find(keys, null)
      const entries = got.map((entry) => keys.decrypt(entry.payload))
      expect(entries.sort()).toEqual(["msg1", "msg2"]) // Sort order of items with the same timestamp is not defined

      // Wait a bit, add a message with a new timestamp and ensure filter works
      await wait(2)
      const msgId = await set(keys, "msg3")
      const filtered = await find(keys, msgId)
      const filteredEntries = filtered.map((entry) => keys.decrypt(entry.payload))
      expect(filteredEntries.sort()).toEqual(["msg3"])

      // Delete it all
      await deleteAccount(keys)
    })
  })
}
