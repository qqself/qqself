import { API, Keys, Request } from "../../bridge/pkg/qqself_client_web_bridge"

export interface APIProvider {
  set: (encryptedPayload: string) => Promise<string>
  find: (encryptedPayload: string) => Promise<EncryptedEntry[]>
  deleteAccount: (keys: Keys) => Promise<void>
}

export interface ApiError {
  timestamp: number
  error_code: string
  error: string
}

export interface EncryptedEntry {
  id: string
  payload: string
}

export class ServerApi implements APIProvider {
  async http(req: Request): Promise<Response> {
    // TODO Requests are coming from WebAssembly bridge with host name already set
    //      here we override it if needed depending on a config. Make it configurable
    //      in the bridge itself
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

  async set(encryptedPayload: string): Promise<string> {
    const resp = await this.http(API.createApiSetRequest(encryptedPayload))
    return resp.text()
  }

  async find(encryptedPayload: string): Promise<EncryptedEntry[]> {
    const resp = await this.http(API.createApiFindRequest(encryptedPayload))
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

  async deleteAccount(keys: Keys): Promise<void> {
    await this.http(API.createApiDeleteRequest(keys))
  }
}

if (import.meta.vitest) {
  const { describe, test, expect } = import.meta.vitest

  const wait = (seconds: number) => new Promise((resolve) => setTimeout(resolve, seconds * 1000))

  const api = new ServerApi()

  describe("API", () => {
    test("Create new keys", () => {
      const keys = Keys.createNewKeys()
      expect(keys).toBeTruthy()
    })

    test("API", async () => {
      // First find call no data
      const keys = Keys.createNewKeys()
      const lines = await api.find(keys.sign_find_token())
      expect(lines).toEqual([])

      // Add couple of messages
      await api.set(keys.encrypt("msg1"))
      await api.set(keys.encrypt("msg2"))

      // Get all messages back
      const got = await api.find(keys.sign_find_token())
      const entries = got.map((entry) => keys.decrypt(entry.payload))
      expect(entries.sort()).toEqual(["msg1", "msg2"]) // Sort order of items with the same timestamp is not defined

      // Wait a bit, add a message with a new timestamp and ensure filter works
      await wait(2)
      const msgId = await api.set(keys.encrypt("msg3"))
      await wait(2)
      await api.set(keys.encrypt("msg4"))
      const filtered = await api.find(keys.sign_find_token(msgId))
      const filteredEntries = filtered.map((entry) => keys.decrypt(entry.payload))
      expect(filteredEntries.sort()).toEqual(["msg4"])

      // Delete it all
      await api.deleteAccount(keys)
    })
  })
}
