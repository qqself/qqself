import { expect, test } from "vitest"
import { Events, Store } from "./store"
import { APIProvider, ServerApi } from "./api"
import { KeyPrefixes } from "./data"

class TestStore extends Store {
  gotEvents = new Map()

  constructor(api?: APIProvider) {
    super(api ?? new ServerApi())
  }

  override async dispatch<T extends keyof Events>(event: T, eventArgs: Events[T]): Promise<void> {
    this.gotEvents.set(event, eventArgs)
    return super.dispatch(event, eventArgs)
  }

  async dispatchAndExpect<T1 extends keyof Events, T2 extends keyof Events>(
    event: T1,
    eventArgs: Events[T1],
    expectedEvent: T2,
    expectedEventArgs?: Events[T2]
  ): Promise<void> {
    this.gotEvents = new Map()
    await this.dispatch(event, eventArgs)
    if (!this.gotEvents.has(expectedEvent)) {
      // Fail if expected event didn't occur
      expect([...this.gotEvents.keys()]).toContain(expectedEvent)
    }
    if (expectedEventArgs) {
      // Check for event argument if we check for those
      expect(this.gotEvents.get(expectedEvent)).toEqual(expectedEventArgs)
    }
  }
}

class OfflineApi implements APIProvider {
  set() {
    return Promise.reject()
  }
  find() {
    return Promise.reject()
  }
  deleteAccount() {
    return Promise.reject()
  }
}

test("Initialization should set user to not authenticated", async () => {
  const store = new TestStore()
  await store.dispatchAndExpect("init.started", null, "auth.login.notAuthenticated")
})

test("Registration should automatically login user", async () => {
  const store1 = new TestStore()
  await store1.dispatchAndExpect("init.started", null, "auth.login.notAuthenticated")
  await store1.dispatchAndExpect(
    "auth.registration.started",
    { mode: "automatic" },
    "auth.login.succeeded"
  )
  expect(store1.userState.encryptionPool).toBeTruthy()

  // Next time user should be authenticated automatically
  const store2 = new TestStore()
  await store2.dispatchAndExpect("init.started", null, "auth.login.succeeded")
  expect(store2.userState.encryptionPool).toBeTruthy()

  // But after logout cached credentials are removed
  await store2.dispatchAndExpect("auth.logout.started", null, "auth.logout.succeeded")
  const store3 = new TestStore()
  await store3.dispatchAndExpect("init.started", null, "auth.login.notAuthenticated")
})

test("On login fetch entries", async () => {
  const store1 = new TestStore()
  await store1.dispatch("init.started", null)
  await store1.dispatch("auth.registration.started", { mode: "automatic" })
  await store1.dispatchAndExpect("data.sync.init", null, "data.sync.succeeded", {
    added: 0,
    fetched: 0,
  })
  // No data by default
  expect(await store1.userState.storage.itemCount()).toEqual(0)

  // Add few remote entries and on next login remote entries should be added
  const entry = "2022-06-07 10:00 11:00 foo"
  await store1.dispatch("data.entry.added", { entry: entry + "1", callSyncAfter: false })
  await store1.dispatchAndExpect(
    "data.entry.added",
    { entry: entry + "2", callSyncAfter: true },
    "data.sync.succeeded",
    { added: 2, fetched: 2 }
  )
  const store2 = new TestStore()
  await store2.dispatch("init.started", null)
  await store2.dispatchAndExpect("data.sync.init", null, "data.sync.succeeded", {
    added: 0,
    fetched: 1, // As there are two entries with the same timestamp we see here another one from what we send via lastKnownId
  })
  const values = await store1.userState.storage.values(KeyPrefixes.EntryRemote)
  expect(values.map((v) => v.value).sort()).toEqual([entry + "1", entry + "2"])
})

test("Status pending when new local entries exists", async () => {
  const store = new TestStore()
  await store.dispatch("init.started", null)
  await store.dispatch("auth.registration.started", { mode: "automatic" })
  await store.dispatchAndExpect(
    "data.entry.added",
    { entry: "2022-06-07 10:00 11:00 foo", callSyncAfter: false },
    "status.sync",
    { status: "pending" }
  )
  const checkEntries = async (entries: { local: number; remote: number }) => {
    const storage = store.userState.storage
    expect(await storage.values(KeyPrefixes.EntryLocal)).toHaveLength(entries.local)
    expect(await storage.values(KeyPrefixes.EntryRemote)).toHaveLength(entries.remote)
  }
  await checkEntries({ local: 1, remote: 0 })
  await store.dispatchAndExpect("data.sync.started", null, "status.sync", { status: "completed" })
  await checkEntries({ local: 0, remote: 1 })
})

test("Status remains pending when sending failed", async () => {
  const store = new TestStore(new OfflineApi())
  await store.dispatch("init.started", null)
  await store.dispatch("auth.registration.started", { mode: "automatic" })
  await store.dispatchAndExpect(
    "data.entry.added",
    { entry: "2022-06-07 10:00 11:00 foo", callSyncAfter: true },
    "status.sync",
    { status: "pending" }
  )
})
