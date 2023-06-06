import localforage from "localforage"

type Provider =
  | { type: "persisted"; provider: LocalForage }
  | { type: "memory"; provider: { [key: string]: string } }

export class Storage {
  private storage: Provider

  private constructor(storage: Provider) {
    this.storage = storage
  }

  // Initializes new storage with specified name. Creates a new one if doesn't exists
  // Set persisted=false for in memory storage
  static async init(name: string, persisted: boolean): Promise<Storage> {
    if (persisted) {
      return new Storage({
        type: "persisted" as const,
        provider: localforage.createInstance({ name: name }),
      })
    } else {
      return new Storage({ type: "memory" as const, provider: {} })
    }
  }

  static async initDefault(persisted: boolean): Promise<Storage> {
    return Storage.init("DEFAULT", persisted)
  }

  // Cleans up the storage by removing all the keys and values
  async clear(): Promise<void> {
    if (this.storage.type == "memory") {
      this.storage.provider = {}
    } else {
      await this.storage.provider.clear()
    }
  }

  // Returns number of keys in a storage
  async itemCount(): Promise<number> {
    if (this.storage.type == "memory") {
      return Object.keys(this.storage.provider).length
    } else {
      return this.storage.provider.length()
    }
  }

  // Returns value for the specified key. Returns null if key no found
  async getItem(key: string): Promise<string | null> {
    if (this.storage.type == "memory") {
      return this.storage.provider[key] || null
    } else {
      return this.storage.provider.getItem(key)
    }
  }

  // Sets a value for the specified key
  async setItem(key: string, value: string): Promise<void> {
    if (this.storage.type == "memory") {
      this.storage.provider[key] = value
    } else {
      await this.storage.provider.setItem(key, value)
    }
  }

  // Returns all existing key/value pairs sorted by the key
  async values(): Promise<{ key: string; value: string }[]> {
    if (this.storage.type == "memory") {
      return Object.entries(this.storage.provider)
        .map(([key, value]) => ({ key, value }))
        .sort((a, b) => a.key.localeCompare(b.key))
    } else {
      const items: { key: string; value: string }[] = []
      await this.storage.provider.iterate((value: string, key) => {
        items.push({ key, value })
      })
      return items
    }
  }
}

if (import.meta.vitest) {
  const { describe, test, expect } = import.meta.vitest

  describe("storage", () => {
    test("getItem - setItem", async () => {
      const storage = await Storage.init("test", false)
      expect(await storage.getItem("foo")).toBe(null)
      await storage.setItem("foo", "bar")
      expect(await storage.getItem("foo")).toBe("bar")
    })

    test("values", async () => {
      const storage = await Storage.init("test", false)
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
      const storage = await Storage.init("test", false)
      expect(await storage.itemCount()).toBe(0)
      await storage.setItem("foo", "bar")
      expect(await storage.itemCount()).toBe(1)
      await storage.setItem("bar", "foo")
      expect(await storage.itemCount()).toBe(2)
    })

    test("clear", async () => {
      const storage = await Storage.init("test", false)
      expect(await storage.itemCount()).toBe(0)
      await storage.setItem("foo", "bar")
      expect(await storage.itemCount()).toBe(1)
      await storage.clear()
      expect(await storage.itemCount()).toBe(0)
    })
  })
}
