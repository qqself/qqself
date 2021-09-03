interface EntryStorage {
    read() : Promise<string>
    write(data: string) : Promise<void>
}

export class MemoryStorage implements EntryStorage {
    key = "data"

    async read() : Promise<string> {
        const data = window.localStorage.getItem(this.key)
        if (!data) {
            return ""
        }
        return data
    }

    async write(data: string) : Promise<void> {
        window.localStorage.setItem(this.key, data)
    }
}
