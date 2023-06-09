import { html, LitElement } from "lit"
import { customElement, state } from "lit/decorators.js"
import "./pages/loading"
import "./pages/register"
import "./pages/login"
import "./pages/progress"
import { App, Keys } from "../bridge/pkg"
import { EncryptionPool } from "./encryptionPool/pool"
import { Storage } from "./storage"
import { LoadedEvent } from "./pages/loading"
import { LoggedInEvent } from "./pages/login"

type Page = "login" | "register" | "progress" | "devcards"

interface State {
  initComplete: boolean
  page: Page
  keys: Keys | null
  encryptionPool: EncryptionPool | null
  app: App | null
}

const defaultState: State = {
  initComplete: false,
  page: "login",
  keys: null,
  encryptionPool: null,
  app: null,
}

@customElement("q-main")
export class Main extends LitElement {
  @state()
  state = defaultState

  async firstUpdated() {
    const availablePages = ["login", "register", "progress"]
    const page = window.location.hash.slice(1)
    if (availablePages.includes(page)) {
      if (page != "progress" || this.state.keys) {
        this.state.page = page as Page // Show progress only when keys are available
      } else {
        this.moveToPage("login")
      }
    }
    if (import.meta.env.DEV) {
      // devcards enabled only in dev
      if (page.startsWith("devcards")) {
        await import("./pages/devcards")
        this.state.page = "devcards"
      }
    }
  }

  moveToPage(page: Page) {
    window.history.pushState(null, "", "#" + page)
    this.state = { ...this.state, page }
  }

  onLoadCompleted(sender: LoadedEvent) {
    if (sender.detail.keys) {
      const app = App.new(sender.detail.keys)
      // There are cached keys, move to progress page right away
      this.state = {
        ...this.state,
        initComplete: true,
        encryptionPool: EncryptionPool.initWithKeys(sender.detail.keys),
        keys: sender.detail.keys,
        page: "progress",
        app,
      }
    } else {
      // No cached keys - continue with normal login flow
      this.state = {
        ...this.state,
        initComplete: true,
      }
    }
  }

  async onLoginCompleted(sender: LoggedInEvent) {
    const keys = sender.detail.keys
    const storage = await Storage.initDefault(true)
    await storage.setItem("keys", keys.serialize())
    const app = App.new(keys)
    this.state = { ...this.state, keys, app, encryptionPool: EncryptionPool.initWithKeys(keys) }
    this.moveToPage("progress")
  }

  render() {
    if (!this.state.initComplete) {
      return html`<q-loading-page @loaded=${this.onLoadCompleted} />`
    }
    switch (this.state.page) {
      case "login": {
        return html`<q-login-page
          .keys=${this.state.keys}
          .encryptionPool=${this.state.encryptionPool}
          @loggedIn=${this.onLoginCompleted}
          @register=${() => this.moveToPage("register")}
        />`
      }
      case "register": {
        return html`<q-register-page @registered=${() => this.moveToPage("login")} />`
      }
      case "progress": {
        return html`<q-progress-page
          .keys=${this.state.keys}
          .app=${this.state.app}
          .encryptionPool=${this.state.encryptionPool}
        />`
      }
      case "devcards": {
        {
          if (import.meta.env.DEV) {
            return html`<q-devcards-page
              .encryptionPool=${this.state.encryptionPool}
            ></q-devcards-page>`
          } else {
            throw new Error("Devcards should not be available in production")
          }
        }
      }
    }
  }
}
