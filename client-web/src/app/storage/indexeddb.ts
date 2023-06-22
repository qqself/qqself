import localforage from "localforage"
import { Storage } from "./storage"

export class IndexedDbStorage implements Storage {
  db: LocalForage

  constructor(dbName: string) {
    this.db = localforage.createInstance({ name: dbName })
  }

  clear(): Promise<void> {
    return this.db.clear()
  }

  itemCount(): Promise<number> {
    return this.db.length()
  }

  getItem(key: string): Promise<string | null> {
    return this.db.getItem(key)
  }

  setItem(key: string, value: string): Promise<void> {
    return this.db.setItem(key, value) as Promise<never>
  }

  removeItem(key: string): Promise<void> {
    return this.db.removeItem(key)
  }

  // TODO localforage unfortunately doesn't support query by prefixes.
  //      It's rather tiny wrapper on top of IndexedDb, so we should drop it and create our own
  async values(keyPrefix: string | ""): Promise<{ key: string; value: string }[]> {
    const items: { key: string; value: string }[] = []
    await this.db.iterate((value: string, key) => {
      if (!key.startsWith(keyPrefix)) return
      items.push({ key, value })
    })
    return items
  }
}
