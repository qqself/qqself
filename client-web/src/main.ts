import { html, LitElement } from "lit"
import { customElement, state } from "lit/decorators.js"
import "./ui/pages/loading"
import "./ui/pages/register"
import "./ui/pages/login"
import "./ui/pages/progress"
import { Store } from "./app/store"
import { DateDay } from "../bridge/pkg/qqself_client_web_bridge"

type Page = "loading" | "login" | "register" | "progress" | "devcards"

@customElement("q-main")
export class Main extends LitElement {
  @state()
  page: Page = "loading"

  store = new Store()

  async firstUpdated() {
    if (import.meta.env.DEV) {
      // If we are in dev mode and devcards are requested simply render it
      const page = window.location.hash.slice(1)
      if (page.startsWith("devcards")) {
        await import("./ui/pages/devcards")
        this.page = "devcards"
        return
      }
    }

    this.store.subscribe("auth.login.notAuthenticated", () => (this.page = "login"))
    this.store.subscribe("auth.login.succeeded", () => (this.page = "progress"))
    this.store.subscribe("auth.registration.started", () => (this.page = "register"))
  }

  render() {
    switch (this.page) {
      case "loading":
        return html`<q-loading-page .store=${this.store} />`
      case "devcards":
        if (import.meta.env.DEV) {
          return html`<q-devcards-page .store=${this.store} />`
        } else {
          throw new Error("Devcards should not be available in production")
        }
      case "login":
        return html`<q-login-page .store=${this.store} />`
      case "progress":
        return html`<q-progress-page
          .store=${this.store}
          .currentDay=${DateDay.fromDate(new Date())}
        />`
      case "register":
        return html`<q-register-page .store=${this.store} />`
    }
  }
}
