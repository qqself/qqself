import localforage from "localforage"
import { StorageProvider } from "./storage"

export class IndexedDbStorage implements StorageProvider {
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

  async values(): Promise<{ key: string; value: string }[]> {
    const items: { key: string; value: string }[] = []
    await this.db.iterate((value: string, key) => {
      items.push({ key, value })
    })
    return items
  }
}
