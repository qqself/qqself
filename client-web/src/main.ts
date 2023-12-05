import "./ui/pages/loading"
import "./ui/pages/register"
import "./ui/pages/login"
import "./ui/pages/progress"
import "./ui/pages/growth"

import { css, html, LitElement } from "lit"
import { customElement, state } from "lit/decorators.js"

import { DateDay } from "../qqself_core"
import { ServerApi } from "./app/api"
import { Store } from "./app/store"
import { colors } from "./ui/styles"

type Page = "loading" | "login" | "register" | "progress" | "growth" | "devcards"

@customElement("q-main")
export class Main extends LitElement {
  @state()
  page: Page = "loading"

  store = new Store(new ServerApi(import.meta.env.VITE_API_HOST))

  static styles = css`
    .root {
      background-color: ${colors.background.light};
      height: 100%;
      padding-top: 20px;
    }
  `

  async firstUpdated() {
    if (import.meta.env.DEV) {
      // If we are in dev mode and devcards are requested simply render it
      const page = window.location.hash.slice(1)
      if (page.startsWith("devcards")) {
        await import("./ui/pages/devcards")
        this.page = "devcards"
        return
      } else if (page.startsWith("growth")) {
        // TODO Temporary for testing new `growth` page
        this.page = "growth"
        return
      }
    }

    this.store.subscribe("auth.login.notAuthenticated", () => (this.page = "login"))
    this.store.subscribe("auth.login.succeeded", () => (this.page = "progress"))
    this.store.subscribe("auth.registration.started", () => (this.page = "register"))
  }

  renderPage() {
    switch (this.page) {
      case "loading":
        return html`<q-loading-page .store=${this.store} />`
      case "devcards":
        if (import.meta.env.DEV) {
          return html`<q-devcards-page />`
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
      case "growth":
        return html`<q-growth-page .store=${this.store} />`
    }
  }

  render() {
    return html`<div class="root">${this.renderPage()}</div>`
  }
}
