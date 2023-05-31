import localforage from "localforage"

type StorageProvider =
  | { type: "persisted"; provider: LocalForage }
  | { type: "memory"; provider: { [key: string]: string } }

export class Cache {
  private storage: StorageProvider

  private constructor(storage: StorageProvider) {
    this.storage = storage
  }

  // Initializes new cache with specified name. Creates a new one if doesn't exists
  // Set persisted=false for in memory cache
  static async init(cacheName: string, persisted: boolean): Promise<Cache> {
    if (persisted) {
      return new Cache({
        type: "persisted" as const,
        provider: localforage.createInstance({ name: cacheName }),
      })
    } else {
      return new Cache({ type: "memory" as const, provider: {} })
    }
  }

  // Cleans up the cache by removing all the keys and values
  async clear(): Promise<void> {
    if (this.storage.type == "memory") {
      this.storage.provider = {}
    } else {
      await this.storage.provider.clear()
    }
  }

  // Returns number of keys in a cache
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

  describe("cache", () => {
    test("getItem - setItem", async () => {
      const cache = await Cache.init("test", false)
      expect(await cache.getItem("foo")).toBe(null)
      await cache.setItem("foo", "bar")
      expect(await cache.getItem("foo")).toBe("bar")
    })

    test("values", async () => {
      const cache = await Cache.init("test", false)
      expect(await cache.values()).toEqual([])
      const data = [
        { key: "foo1", value: "bar1" },
        { key: "foo3", value: "bar3" },
        { key: "foo2", value: "bar2" },
      ]
      for (const { key, value } of data) {
        await cache.setItem(key, value)
      }
      // Values should be sorted by the key
      expect(await cache.values()).toEqual(data.sort((a, b) => a.key.localeCompare(b.key)))
    })

    test("count", async () => {
      const cache = await Cache.init("test", false)
      expect(await cache.itemCount()).toBe(0)
      await cache.setItem("foo", "bar")
      expect(await cache.itemCount()).toBe(1)
      await cache.setItem("bar", "foo")
      expect(await cache.itemCount()).toBe(2)
    })

    test("clear", async () => {
      const cache = await Cache.init("test", false)
      expect(await cache.itemCount()).toBe(0)
      await cache.setItem("foo", "bar")
      expect(await cache.itemCount()).toBe(1)
      await cache.clear()
      expect(await cache.itemCount()).toBe(0)
    })
  })
}
