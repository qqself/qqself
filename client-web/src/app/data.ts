import { DateDay, stringHash, UiRecord } from "../../bridge/pkg"
import { APIProvider } from "../app/api"
import { trace } from "../logger"
import { isBrowser } from "../utils"
import { Store } from "./store"

// We have one KV storage, to differentiate values we are using different prefixes for the keys
export const KeyPrefixes = {
  EntryLocal: "entry.local.",
  EntryRemote: "entry.remote.",
  LastSync: "sync.lastId.",
}

export class DataEvents {
  private lastSync: Date | null = null
  private store: Store
  private api: APIProvider

  constructor(store: Store, api: APIProvider) {
    this.store = store
    this.api = api
    setInterval(() => this.onTimerTick.bind(this), 60 * 1000)
    if (isBrowser) {
      window.addEventListener("online", () => {
        trace("DataEvents.window.online")
        void store.dispatch("data.sync.becomeOnline", null)
      })
    }
  }

  async onSyncInit() {
    trace(`DataEvents loading cached entries`)
    const storage = this.store.userState.storage
    let loadedRemote = 0
    let loadedLocal = 0
    for (const entry of await storage.values(KeyPrefixes.EntryRemote)) {
      const record = UiRecord.parse(entry.value)
      this.store.userState.views.add_record(record, false)
      loadedRemote++
    }
    for (const entry of await storage.values(KeyPrefixes.EntryLocal)) {
      const record = UiRecord.parse(entry.value)
      this.store.userState.views.add_record(record, false)
      loadedLocal++
    }
    trace(`DataEvents loaded cached data: remote=${loadedRemote}, local=${loadedLocal}`)
    if (loadedLocal) {
      // There are pending local changes, update the status
      await this.store.dispatch("status.sync", { status: "pending" })
    }
    return this.store.dispatch("data.sync.started", null)
  }

  onSyncOutdated(): Promise<void> {
    return this.store.dispatch("data.sync.started", null)
  }

  onBecomeOnline(): Promise<void> {
    return this.store.dispatch("data.sync.started", null)
  }

  async onSyncStarted(): Promise<void> {
    try {
      const start = performance.now()
      const sentToServer = await this.sendLocalChanges()
      await this.store.dispatch("status.sync", { status: "completed" })
      await this.store.dispatch("status.currentOperation", { operation: "Fetching entries" })
      const receivedFromServer = await this.receiveRemoteChanges()
      trace(`DataEvents sync completed in: ${Math.floor(performance.now() - start)}ms`)
      await this.store.dispatch("status.currentOperation", { operation: null })
      return this.store.dispatch("data.sync.succeeded", {
        added: sentToServer,
        fetched: receivedFromServer,
      })
    } catch (ex) {
      // TODO While it may be handy to show an error to the user when we going to hide it?
      await this.store.dispatch("status.currentOperation", {
        operation: "Sync error: " + String(ex),
      })
      return this.store.dispatch("data.sync.errored", { error: ex as Error })
    }
  }

  async onEntryAdded(entry: string, callSyncAfter: boolean): Promise<void> {
    const record = UiRecord.parse(entry)
    this.store.userState.views.add_record(record, true, DateDay.fromDate(new Date()))
    await this.addEntryToCache(entry, null)
    await this.store.dispatch("status.sync", { status: "pending" })
    if (callSyncAfter) {
      return this.store.dispatch("data.sync.started", null)
    }
  }

  private onTimerTick() {
    trace("DataEvents.onTimerTick")
    // If we didn't do any sync then ignore timer
    if (!this.lastSync) return

    // Emit outdated event if last sync happened more than an hour ago
    const lastSyncAge = new Date().getTime() - this.lastSync.getTime()
    if (lastSyncAge > 1000 * 60 * 60) {
      void this.store.dispatch("data.sync.outdated", { lastSync: this.lastSync })
    }
  }

  private async sendLocalChanges(): Promise<number> {
    const storage = this.store.userState.storage
    const localEntries = await storage.values(KeyPrefixes.EntryLocal)
    if (!localEntries.length) {
      trace(`DataEvents no pending changes`)
      return 0
    }

    trace(`DataEvents sending ${localEntries.length} entries`)
    // TODO Do encryption and sending in parallel and in batches of N
    // TODO If there is no internet then we would keep repeating encryption for entries, maybe cache it somewhere?
    const sendStart = performance.now()
    for (const entry of localEntries) {
      const payload = await this.store.userState.encryptionPool.encrypt(entry.value)
      const remoteEntryId = await this.api.set(payload.payload)
      // TODO Storage should support transactional change of the key
      const newId = KeyPrefixes.EntryRemote + remoteEntryId
      await storage.setItem(newId, entry.value)
      await storage.removeItem(entry.key)
      trace(`DataEvents local entry saved: ${entry.key} -> ${newId}`)
    }
    trace(`DataEvents sending finished in:${Math.floor(performance.now() - sendStart)}ms`)
    return localEntries.length
  }

  private async receiveRemoteChanges(): Promise<number> {
    const start = performance.now()
    const lastSyncId = await this.loadLastSyncId()
    trace(`DataEvents sending find request, lastSyncId=${String(lastSyncId)}`)
    const findToken = await this.store.userState.encryptionPool.sign({ kind: "find", lastSyncId })
    const remoteEntries = await this.api.find(findToken)
    const requestFinished = performance.now()
    const decrypted = await this.store.userState.encryptionPool.decryptAll(remoteEntries)
    for (const entry of decrypted) {
      const record = UiRecord.parse(entry.text)
      this.store.userState.views.add_record(record, false)
      await this.addEntryToCache(entry.text, entry.id)
    }
    if (decrypted.length > 1) {
      await this.saveLastSyncId(decrypted[decrypted.length - 1].id)
    }
    const end = performance.now()
    trace(
      `DataEvents ${decrypted.length} entries loaded in ${Math.floor(
        end - start,
      )}ms. API=${Math.floor(requestFinished - start)}ms Decryption=${Math.floor(
        end - requestFinished,
      )}ms`,
    )
    return decrypted.length
  }

  private addEntryToCache(entry: string, entryId: string | null): Promise<void> {
    const key = entryId
      ? KeyPrefixes.EntryRemote + entryId
      : KeyPrefixes.EntryLocal + stringHash(entry)
    return this.store.userState.storage.setItem(key, entry)
  }

  private saveLastSyncId(lastId: string): Promise<void> {
    return this.store.userState.storage.setItem(KeyPrefixes.LastSync, lastId)
  }

  private loadLastSyncId(): Promise<string | null> {
    return this.store.userState.storage.getItem(KeyPrefixes.LastSync)
  }
}
