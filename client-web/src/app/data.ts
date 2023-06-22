import { trace, warn } from "../logger"
import { isBrowser } from "../utils"
import { Store } from "./store"
import { stringHash } from "../../bridge/pkg"
import * as API from "../app/api"

// We have one KV storage, to differentiate values we are using different prefixes for the keys
const KeyPrefixes = {
  EntryLocal: "entry.local.",
  EntryRemote: "entry.remote.",
  LastSync: "sync.lastId.",
}

export class DataEvents {
  private lastSync: Date | null = null
  private store: Store

  constructor(store: Store) {
    this.store = store
    setInterval(() => this.onTimerTick.bind(this), 60 * 1000)
    if (isBrowser) {
      window.addEventListener("online", () => {
        trace("DataEvents.window.online")
        void store.dispatch("data.sync.becomeOnline", null)
      })
    }
  }

  async onLoadCached() {
    trace(`DataEvents loading cached entries`)
    const storage = this.store.userState.storage
    let loadedRemote = 0
    let loadedLocal = 0
    for (const entry of await storage.values(KeyPrefixes.EntryRemote)) {
      this.store.userState.views.add_entry(entry.value)
      loadedRemote++
    }
    for (const entry of await storage.values(KeyPrefixes.EntryLocal)) {
      this.store.userState.views.add_entry(entry.value)
      loadedLocal++
    }
    trace(`DataEvents loaded cached data: remote=${loadedRemote}, local=${loadedLocal}`)
  }

  onSyncOutdated(): Promise<void> {
    return this.store.dispatch("data.sync.started", null)
  }

  onBecomeOnline(): Promise<void> {
    return this.store.dispatch("data.sync.started", null)
  }

  async onSyncStarted(): Promise<void> {
    const start = performance.now()
    const sentToServer = await this.sendLocalChanges()
    const receivedFromServer = await this.receiveRemoteChanges()
    const syncTime = `${Math.floor(performance.now() - start)}ms`
    return this.store.dispatch("data.sync.succeeded", {
      duration: syncTime,
      added: sentToServer,
      fetched: receivedFromServer,
    })
  }

  async onEntryAdded(entry: string, callSyncAfter: boolean): Promise<void> {
    this.store.userState.views.add_entry(entry)
    await this.addEntryToCache(entry, null)
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
      try {
        const remoteEntryId = await API.set(payload.payload)
        // TODO Storage should support transactional change of the key
        const newId = KeyPrefixes.EntryRemote + remoteEntryId
        await storage.setItem(newId, entry.value)
        await storage.removeItem(entry.key)
        trace(`DataEvents local entry saved: ${entry.key} -> ${newId}`)
      } catch (ex) {
        warn(`DataEvents - sync failed: ${String(ex)}`)
        void this.store.dispatch("data.sync.errored", { error: ex as Error })
        throw ex
      }
    }
    trace(`DataEvents sending finished in:${Math.floor(performance.now() - sendStart)}ms`)
    return localEntries.length
  }

  private async receiveRemoteChanges(): Promise<number> {
    trace(`DataEvents sending find request`)
    const start = performance.now()
    const lastSyncId = await this.loadLastSyncId()
    const findToken = await this.store.userState.encryptionPool.sign({ kind: "find", lastSyncId })
    const remoteEntries = await API.find(findToken)
    const requestFinished = performance.now()
    const decrypted = await this.store.userState.encryptionPool.decryptAll(remoteEntries)
    for (const entry of decrypted) {
      this.store.userState.views.add_entry(entry.text)
      await this.addEntryToCache(entry.text, entry.id)
    }
    if (decrypted.length > 1) {
      await this.saveLastSyncId(decrypted[decrypted.length - 1].id)
    }
    const end = performance.now()
    trace(
      `DataEvents ${decrypted.length} entries loaded in ${Math.floor(
        end - start
      )}ms. API=${Math.floor(requestFinished - start)}ms Decryption=${Math.floor(
        end - requestFinished
      )}ms`
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
