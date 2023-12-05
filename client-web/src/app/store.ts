import { Cryptor, Views } from "../../qqself_core"
import { debug, info } from "../logger"
import { APIProvider } from "./api"
import * as Auth from "./auth"
import { CryptorPool } from "./cryptorPool/pool"
import { DataEvents } from "./data"
import * as Init from "./init"
import { Storage } from "./storage/storage"

export interface QueryResultsUpdate {
  view: "QueryResults"
}
export interface SkillsViewUpdate {
  view: "Skills"
  message: string
}
export interface WeekUpdate {
  view: "Week"
}
export type ViewUpdate = QueryResultsUpdate | SkillsViewUpdate | WeekUpdate
export interface SkillsViewNotification {
  view: "Skills"
  message: string
}
export type ViewNotification = SkillsViewNotification

// Events are application wide activities that causes some side effect
export interface Events {
  // Init
  "init.started": null
  "init.succeeded": { cryptor: Cryptor | null }
  "init.errored": { error: Error }
  // Auth
  "auth.login.started": { serializedKeys: string }
  "auth.login.notAuthenticated": null
  "auth.login.succeeded": { cryptor: Cryptor }
  "auth.login.errored": { error: Error }
  "auth.registration.started": { mode: "interactive" | "automatic" }
  "auth.registration.succeeded": { cryptor: Cryptor }
  "auth.logout.started": null
  "auth.logout.succeeded": null
  // Data
  "data.entry.added": { entry: string; callSyncAfter: boolean } // User entered new entry
  "data.sync.init": null // Load cached data from local storage and perform data sync
  "data.sync.outdated": { lastSync: Date } // Last sync happened too long time ago
  "data.sync.becomeOnline": null // App become online after being offline
  "data.sync.started": null // Data sync started because of some conditions or requested manually
  "data.sync.errored": { error: Error } // Data sync finished with an error
  "data.sync.succeeded": { added: number; fetched: number } // Data sync succeeded
  // Status
  "status.sync": { status: "pending" | "completed" } // Sets current sync status
  "status.currentOperation": { operation: string | null } // Sets current long-time operation
  // Views
  "views.update.queryResults": { update: QueryResultsUpdate } // QueryResults view updated
  "views.update.skills": { update: SkillsViewUpdate } // Skills view updated
  "views.update.week": { update: WeekUpdate } // Week view updated
  "views.queryResults.queryUpdated": { query: string } // QueryResults query updated
  "views.notification.skills": { update: SkillsViewNotification } // Notification from Skills view
}

export class Store {
  private eventTarget = new EventTarget()

  api: APIProvider

  constructor(api: APIProvider) {
    debug("Store created")
    this.api = api
  }

  userState!: {
    encryptionPool: CryptorPool
    storage: Storage
    views: Views
    dataEvents: DataEvents
  }

  async dispatch<T extends keyof Events>(event: T, eventArgs: Events[T]): Promise<void> {
    if (!event.startsWith("views.update.")) {
      info(`Event ${event}`) // Filter out noisy views.update
    }
    // TODO TypeScript failed to recognize exact type of eventArgs and keeps it generic
    //      Probably it may be possible to create type helper to avoid event name repetition
    if (event == "init.started") {
      await Init.started(this)
    } else if (event == "init.succeeded") {
      await Init.succeeded(this, (eventArgs as Events["init.succeeded"]).cryptor)
    } else if (event == "auth.login.started") {
      await Auth.login(this, (eventArgs as Events["auth.login.started"]).serializedKeys)
    } else if (event == "auth.login.succeeded") {
      await Auth.loginSucceeded(this, (eventArgs as Events["auth.login.succeeded"]).cryptor)
    } else if (event == "auth.registration.started") {
      await Auth.registrationStarted(this, (eventArgs as Events["auth.registration.started"]).mode)
    } else if (event == "auth.registration.succeeded") {
      await Auth.registrationSucceeded(
        this,
        (eventArgs as Events["auth.registration.succeeded"]).cryptor,
      )
    } else if (event == "auth.logout.started") {
      await Auth.logoutStarted(this)
    } else if (event == "auth.logout.succeeded") {
      await Auth.logoutSucceeded(this)
    } else if (event == "data.sync.becomeOnline") {
      await this.userState.dataEvents.onBecomeOnline()
    } else if (event == "data.sync.outdated") {
      await this.userState.dataEvents.onSyncOutdated()
    } else if (event == "data.sync.started") {
      await this.userState.dataEvents.onSyncStarted()
    } else if (event == "data.sync.init") {
      await this.userState.dataEvents.onSyncInit()
    } else if (event == "data.entry.added") {
      const args = eventArgs as Events["data.entry.added"]
      await this.userState.dataEvents.onEntryAdded(args.entry, args.callSyncAfter)
    } else if (event == "views.queryResults.queryUpdated") {
      const args = eventArgs as Events["views.queryResults.queryUpdated"]
      this.userState.views.update_query(args.query)
    }
    this.eventTarget.dispatchEvent(new CustomEvent(event, { detail: eventArgs }))
    return Promise.resolve()
  }

  subscribe<T extends keyof Events>(
    eventName: T,
    handler: (eventArgs: Events[T]) => unknown,
  ): () => Promise<void> {
    this.eventTarget.addEventListener(eventName, (event: Event) => {
      const customEvent = event as CustomEvent<Events[T]>
      handler(customEvent.detail)
    })
    return () => Promise.resolve(this.eventTarget.removeEventListener(eventName, handler as never))
  }
}
