import { Storage } from "./storage"

// Global mutable variable to store the data while process is running
// to behave closer to persistent storage and make testing easier
const data: { [dbName: string]: { [key: string]: string } } = {}

export class MemoryStorage implements Storage {
  dbName: string

  constructor(dbName: string) {
    this.dbName = dbName
    if (!data[dbName]) {
      data[dbName] = {}
    }
  }

  clear(): Promise<void> {
    data[this.dbName] = {}
    return Promise.resolve()
  }

  itemCount(): Promise<number> {
    return Promise.resolve(Object.keys(data[this.dbName]).length)
  }

  getItem(key: string): Promise<string | null> {
    return Promise.resolve(data[this.dbName][key] || null)
  }

  setItem(key: string, value: string): Promise<void> {
    data[this.dbName][key] = value
    return Promise.resolve()
  }

  removeItem(key: string): Promise<void> {
    delete data[this.dbName][key]
    return Promise.resolve()
  }

  values(): Promise<{ key: string; value: string }[]> {
    return Promise.resolve(
      Object.entries(data[this.dbName])
        .map(([key, value]) => ({ key, value }))
        .sort((a, b) => a.key.localeCompare(b.key))
    )
  }
}
