import { Storage } from "./storage"

// Global mutable variable to store the data while process is running
// to behave closer to persistent storage and make testing easier
const data = new Map<string, Map<string, string>>()

export class MemoryStorage implements Storage {
  dbName: string

  constructor(dbName: string) {
    this.dbName = dbName
    if (!data.has(dbName)) {
      data.set(dbName, new Map())
    }
  }

  clear(): Promise<void> {
    data.set(this.dbName, new Map())
    return Promise.resolve()
  }

  itemCount(): Promise<number> {
    return Promise.resolve(this.db().size)
  }

  getItem(key: string): Promise<string | null> {
    return Promise.resolve(this.db().get(key) ?? null)
  }

  setItem(key: string, value: string): Promise<void> {
    this.db().set(key, value)
    return Promise.resolve()
  }

  removeItem(key: string): Promise<void> {
    this.db().delete(key)
    return Promise.resolve()
  }

  values(keyPrefix: string | ""): Promise<{ key: string; value: string }[]> {
    return Promise.resolve(
      Array.from(this.db().entries())
        .map(([key, value]) => ({ key, value }))
        .filter((v) => v.key.startsWith(keyPrefix))
        .sort((a, b) => a.key.localeCompare(b.key)),
    )
  }

  private db() {
    const db = data.get(this.dbName)
    if (db) return db
    throw new Error(`DB ${this.dbName} doesn't exist`)
  }
}
