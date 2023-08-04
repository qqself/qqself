import "../components/logoBlock"

import { html, LitElement } from "lit"
import { customElement, property, state } from "lit/decorators.js"

import { Store } from "../../app/store"

declare global {
  interface HTMLElementTagNameMap {
    "q-loading-page": LoadingPage
  }
}

@customElement("q-loading-page")
export class LoadingPage extends LitElement {
  @state()
  loaded = false

  @state()
  errors = ""

  @property({ type: Object })
  store!: Store

  firstUpdated() {
    this.store.subscribe("init.errored", (args) => {
      this.errors = String(args.error)
    })
    this.store.subscribe("init.succeeded", () => {
      this.loaded = true
    })
    return this.store.dispatch("init.started", null)
  }

  statusElement() {
    if (this.errors) {
      return html`<h1>Error</h1>
        <p>${this.errors}</p>`
    }
    if (!this.loaded) {
      return html`<h1>Loading...</h1>`
    }
    return html`<h1>Ready</h1>`
  }

  render() {
    return html`<q-logo-block>${this.statusElement()}</q-logo-block>`
  }
}
