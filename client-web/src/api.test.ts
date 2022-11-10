import { Keys } from "../core/pkg/qqself_client_web_core"
import * as API from "./api"

describe("API", () => {
  jest.setTimeout(10 * 1000)

  test("Create new keys", async () => {
    const keys = Keys.createNewKeys()
    expect(keys).toBeTruthy()
  })

  test("API", async () => {
    // First find call no data
    const keys = Keys.createNewKeys()
    const lines = await API.find(keys)
    expect(lines).toEqual([])

    // Add couple of messages
    await API.set(keys, "msg1")
    await API.set(keys, "msg2")

    // Get those back
    const got = await API.find(keys)
    const plaintext = got.map((v) => keys.decrypt(v))
    // Sort order of items with the same timestamp is not defined
    expect(plaintext.sort()).toEqual(["msg1", "msg2"])
  })
})
