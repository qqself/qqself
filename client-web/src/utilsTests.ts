import { APIProvider, ServerApi } from "./app/api"
import { Events, Store } from "./app/store"

export class TestStore extends Store {
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
    // Dynamically load `expect` as it's sometimes useful to use TestStore outside of testing context.
    // Importing `vitest` causes errors if imported outside of vitest context
    const { expect } = await import("vitest")
    // TODO: Still it gives warning during production build, accept `expect` as a parameter

    if (!this.gotEvents.has(expectedEvent)) {
      // Fail if expected event didn't occur
      console.log(this.gotEvents.keys())
      expect([...this.gotEvents.keys()]).toContain(expectedEvent)
    }
    if (expectedEventArgs) {
      // Check for event argument if we check for those
      expect(this.gotEvents.get(expectedEvent)).toEqual(expectedEventArgs)
    }
  }
}

export class OfflineApi implements APIProvider {
  set() {
    return Promise.reject(new Error("Connection error"))
  }
  find() {
    return Promise.reject(new Error("Connection error"))
  }
  deleteAccount() {
    return Promise.reject(new Error("Connection error"))
  }
}
