import { Api, Cryptor, Request } from "../../qqself_core"

export interface APIProvider {
  set: (payload: string) => Promise<string>
  find: (payload: string) => Promise<EncryptedEntry[]>
  deleteAccount: (payload: string) => Promise<void>
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
  basePath: string
  cachedApi?: Api

  constructor(basePath: string) {
    this.basePath = basePath
  }

  get api() {
    // We initialize Api lazily and not in constructor as it requires WASM part to be ready
    if (!this.cachedApi) {
      this.cachedApi = new Api(this.basePath)
    }
    return this.cachedApi
  }

  async http(req: Request): Promise<Response> {
    const headers: Record<string, string> = {}
    req.headers.forEach((header) => {
      headers[header.name] = header.value
    })
    const resp = await fetch(req.url, {
      method: "POST",
      body: req.payload,
      headers,
    })
    if (resp.status != 200) {
      const err = (await resp.json()) as ApiError
      throw new Error("API error: " + err.error)
    }
    return resp
  }

  async set(payload: string): Promise<string> {
    const resp = await this.http(this.api.create_set_request(payload))
    return resp.text()
  }

  async find(payload: string): Promise<EncryptedEntry[]> {
    const resp = await this.http(this.api.create_find_request(payload))
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

  async deleteAccount(payload: string): Promise<void> {
    await this.http(this.api.create_delete_request(payload))
  }
}

if (import.meta.vitest) {
  const { describe, test, expect } = import.meta.vitest

  const wait = (seconds: number) => new Promise((resolve) => setTimeout(resolve, seconds * 1000))

  const api = new ServerApi(import.meta.env.VITE_API_HOST)

  describe("API", () => {
    test("Create new cryptor", () => {
      const cryptor = Cryptor.generate_new()
      expect(cryptor).toBeTruthy()
    })

    test("API", async () => {
      // First find call no data
      const cryptor = Cryptor.generate_new()
      const lines = await api.find(cryptor.sign_find_token())
      expect(lines).toEqual([])

      // Add couple of messages
      await api.set(cryptor.encrypt("msg1"))
      await api.set(cryptor.encrypt("msg2"))

      // Get all messages back
      const got = await api.find(cryptor.sign_find_token())
      const entries = got.map((entry) => cryptor.decrypt(entry.payload))
      expect(entries.sort()).toEqual(["msg1", "msg2"]) // Sort order of items with the same timestamp is not defined

      // Wait a bit, add a message with a new timestamp and ensure filter works
      await wait(2)
      const msgId = await api.set(cryptor.encrypt("msg3"))
      await wait(2)
      await api.set(cryptor.encrypt("msg4"))
      const filtered = await api.find(cryptor.sign_find_token(msgId))
      const filteredEntries = filtered.map((entry) => cryptor.decrypt(entry.payload))
      expect(filteredEntries.sort()).toEqual(["msg4"])

      // Delete it all
      await api.deleteAccount(cryptor.sign_delete_token())
    })
  })
}
