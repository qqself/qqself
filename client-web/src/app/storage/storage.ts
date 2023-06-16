import { isBrowser } from "../../utils"
import { IndexedDbStorage } from "./indexeddb"
import { MemoryStorage } from "./memory"

export interface Storage {
  clear(): Promise<void>
  itemCount(): Promise<number>
  getItem(key: string): Promise<string | null>
  setItem(key: string, value: string): Promise<void>
  removeItem(key: string): Promise<void>
  values(): Promise<{ key: string; value: string }[]>
}

export const newStorage = (dbName: string): Storage => {
  if (isBrowser) {
    return new IndexedDbStorage(dbName)
  } else {
    return new MemoryStorage(dbName)
  }
}

export const newDefaultStorage = (): Storage => {
  return newStorage("DEFAULT")
}

if (import.meta.vitest) {
  const { describe, test, expect } = import.meta.vitest

  const createStorage = async () => {
    const storage = newStorage("test")
    await storage.clear() // ensure storage has nothing before the test
    return storage
  }

  describe("storage", () => {
    test("getItem - setItem", async () => {
      const storage = await createStorage()
      expect(await storage.getItem("foo")).toBe(null)
      await storage.setItem("foo", "bar")
      expect(await storage.getItem("foo")).toBe("bar")
    })

    test("values", async () => {
      const storage = await createStorage()
      expect(await storage.values()).toEqual([])
      const data = [
        { key: "foo1", value: "bar1" },
        { key: "foo3", value: "bar3" },
        { key: "foo2", value: "bar2" },
      ]
      for (const { key, value } of data) {
        await storage.setItem(key, value)
      }
      // Values should be sorted by the key
      expect(await storage.values()).toEqual(data.sort((a, b) => a.key.localeCompare(b.key)))
    })

    test("count", async () => {
      const storage = await createStorage()
      expect(await storage.itemCount()).toBe(0)
      await storage.setItem("foo", "bar")
      expect(await storage.itemCount()).toBe(1)
      await storage.setItem("bar", "foo")
      expect(await storage.itemCount()).toBe(2)
    })

    test("clear", async () => {
      const storage = await createStorage()
      expect(await storage.itemCount()).toBe(0)
      await storage.setItem("foo", "bar")
      expect(await storage.itemCount()).toBe(1)
      await storage.clear()
      expect(await storage.itemCount()).toBe(0)
    })

    test("persistence for default", async () => {
      const storage1 = newDefaultStorage()
      await storage1.setItem("foo", "bar")

      const storage2 = newDefaultStorage()
      expect(await storage2.getItem("foo")).toEqual("bar")
    })
  })
}
