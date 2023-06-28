import { ExpectStatic } from "vitest"
import { Keys, Views } from "../../bridge/pkg/qqself_client_web_bridge"
import { EncryptionPool } from "./encryptionPool/pool"
import { Storage } from "./storage/storage"
import * as Auth from "./auth"
import * as Init from "./init"
import { DataEvents, KeyPrefixes } from "./data"
import { debug, info, warn } from "../logger"

// Events are application wide activities that causes some side effect
interface Events {
  // Init
  "init.started": null
  "init.succeeded": { cachedKeys: Keys | null }
  "init.errored": { error: Error }
  // Auth
  "auth.login.started": { keysString: string }
  "auth.login.notAuthenticated": null
  "auth.login.succeeded": { keys: Keys }
  "auth.login.errored": { error: Error }
  "auth.registration.started": { mode: "interactive" | "automatic" }
  "auth.registration.succeeded": { keys: Keys }
  "auth.logout.started": null
  "auth.logout.succeeded": null
  // Data
  "data.entry.added": { entry: string; callSyncAfter: boolean } // User entered new entry
  "data.sync.loadCached": null // Load cached data from storage
  "data.sync.outdated": { lastSync: Date } // Last sync happened too long time ago
  "data.sync.becomeOnline": null // App become online after being offline
  "data.sync.started": null // Data sync started because of some conditions or requested manually
  "data.sync.errored": { error: Error } // Data sync finished with an error
  "data.sync.succeeded": { duration: string; added: number; fetched: number } // Data sync succeeded
}

export class Store {
  private eventTarget = new EventTarget()
  private dataEvents: DataEvents

  constructor() {
    debug("Store created")
    this.dataEvents = new DataEvents(this)
  }

  userState!: {
    encryptionPool: EncryptionPool
    storage: Storage
    views: Views
  }

  async dispatch<T extends keyof Events>(event: T, eventArgs: Events[T]): Promise<void> {
    info(`Event ${event}`)
    // TODO TypeScript failed to recognize exact type of eventArgs and keeps it generic
    //      Probably it may be possible to create type helper to avoid event name repetition
    if (event == "init.started") {
      await Init.started(this)
    } else if (event == "init.succeeded") {
      await Init.succeeded(this, (eventArgs as Events["init.succeeded"]).cachedKeys)
    } else if (event == "auth.login.started") {
      await Auth.login(this, (eventArgs as Events["auth.login.started"]).keysString)
    } else if (event == "auth.login.succeeded") {
      await Auth.loginSucceeded(this, (eventArgs as Events["auth.login.succeeded"]).keys)
    } else if (event == "auth.registration.started") {
      await Auth.registrationStarted(this, (eventArgs as Events["auth.registration.started"]).mode)
    } else if (event == "auth.registration.succeeded") {
      await Auth.registrationSucceeded(
        this,
        (eventArgs as Events["auth.registration.succeeded"]).keys
      )
    } else if (event == "auth.logout.started") {
      await Auth.logoutStarted(this)
    } else if (event == "auth.logout.succeeded") {
      await Auth.logoutSucceeded(this)
    } else if (event == "data.sync.becomeOnline") {
      await this.dataEvents.onBecomeOnline()
    } else if (event == "data.sync.outdated") {
      await this.dataEvents.onSyncOutdated()
    } else if (event == "data.sync.started") {
      await this.dataEvents.onSyncStarted()
    } else if (event == "data.sync.loadCached") {
      await this.dataEvents.onLoadCached()
    } else if (event == "data.entry.added") {
      const args = eventArgs as Events["data.entry.added"]
      await this.dataEvents.onEntryAdded(args.entry, args.callSyncAfter)
    } else {
      warn(`Unhandled event: ${event}`)
    }
    this.eventTarget.dispatchEvent(new CustomEvent(event, { detail: eventArgs }))
    return Promise.resolve()
  }

  subscribe<T extends keyof Events>(
    eventName: T,
    handler: (eventArgs: Events[T]) => unknown
  ): () => Promise<void> {
    this.eventTarget.addEventListener(eventName, (event: Event) => {
      const customEvent = event as CustomEvent<Events[T]>
      handler(customEvent.detail)
    })
    return () => Promise.resolve(this.eventTarget.removeEventListener(eventName, handler as never))
  }
}

class TestStore extends Store {
  expect: ExpectStatic

  constructor(expect: ExpectStatic) {
    super()
    this.expect = expect
  }

  async dispatchAndExpect<T extends keyof Events>(
    event: T,
    eventArgs: Events[T],
    expectedEvent: T
  ): Promise<void> {
    const got = Array<T>()
    this.subscribe(expectedEvent, () => got.push(expectedEvent))
    await this.dispatch(event, eventArgs)
    this.expect(got).toContain(expectedEvent)
  }
}

if (import.meta.vitest) {
  const { test, expect } = import.meta.vitest

  test("Initialization should set user to not authenticated", async () => {
    const store = new TestStore(expect)
    await store.dispatchAndExpect("init.started", null, "auth.login.notAuthenticated")
  })

  test("Registration should automatically login user", async () => {
    const store1 = new TestStore(expect)
    await store1.dispatchAndExpect("init.started", null, "auth.login.notAuthenticated")
    await store1.dispatchAndExpect(
      "auth.registration.started",
      { mode: "automatic" },
      "auth.login.succeeded"
    )
    expect(store1.userState.encryptionPool).toBeTruthy()

    // Next time user should be authenticated automatically
    const store2 = new TestStore(expect)
    await store2.dispatchAndExpect("init.started", null, "auth.login.succeeded")
    expect(store2.userState.encryptionPool).toBeTruthy()

    // But after logout cached credentials are removed
    await store2.dispatchAndExpect("auth.logout.started", null, "auth.logout.succeeded")
    const store3 = new TestStore(expect)
    await store3.dispatchAndExpect("init.started", null, "auth.login.notAuthenticated")
  })

  test("On login fetch entries", async () => {
    const store1 = new TestStore(expect)
    await store1.dispatch("init.started", null)
    await store1.dispatchAndExpect(
      "auth.registration.started",
      { mode: "automatic" },
      "data.sync.succeeded"
    )
    // No data by default
    expect(await store1.userState.storage.itemCount()).toEqual(0)

    // Add few remote entries and on next login remote entries should be added
    const entry = "2022-06-07 10:00 11:00 foo"
    await store1.dispatch("data.entry.added", { entry: entry + "1", callSyncAfter: false })
    await store1.dispatchAndExpect(
      "data.entry.added",
      { entry: entry + "2", callSyncAfter: true },
      "data.sync.succeeded"
    )
    const store2 = new TestStore(expect)
    await store2.dispatchAndExpect("init.started", null, "data.sync.succeeded")
    const values = await store1.userState.storage.values(KeyPrefixes.EntryRemote)
    expect(values.map((v) => v.value).sort()).toEqual([entry + "1", entry + "2"])
  })
}
