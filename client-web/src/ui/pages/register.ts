import "../components/logoBlock"
import "../controls/button"

import { css, html, LitElement } from "lit"
import { customElement, property, state } from "lit/decorators.js"

import { Cryptor } from "../../../qqself_core"
import * as Auth from "../../app/auth"
import { Store } from "../../app/store"

declare global {
  interface HTMLElementTagNameMap {
    "q-register-page": RegisterPage
  }
}

@customElement("q-register-page")
export class RegisterPage extends LitElement {
  @property({ type: Object })
  store!: Store

  @state()
  cryptor: Cryptor | null = null

  @state()
  generating = false

  static styles = css`
    .about-keys {
      max-width: 600px;
      text-align: justify;
    }
    .advice {
      text-align: justify;
      max-width: 600px;
    }

    .login {
      margin-top: 40px;
      display: block;
    }
  `

  async createNewKeys() {
    this.generating = true
    this.cryptor = await Auth.generateCryptor()
    this.generating = false
  }

  createDownloadLink() {
    if (!this.cryptor) throw new Error("Keys should be existed by now")
    const blob = new Blob([this.cryptor.serialize_keys()], { type: "text/plain" })
    return window.URL.createObjectURL(blob)
  }

  onLogin() {
    if (!this.cryptor) throw new Error("Keys should be existed by now")
    return this.store.dispatch("auth.registration.succeeded", { cryptor: this.cryptor })
  }

  renderRegister() {
    return html`
      <q-logo-block>
        <h1>Register</h1>
        <p class="about-keys">
          All the content is encrypted and only you have an access to the keys. We can't read it, we
          can't analyze it or decide to show ads based on that. Downside is if key is lost, then all
          the data is gone. We advice you to store the keys using password manager and store file
          with keys as well.
        </p>
        <q-button @clicked="${this.createNewKeys.bind(this)}">Create new keys</q-button>
      </q-logo-block>
    `
  }

  renderGenerating() {
    return html`
      <q-logo-block>
        <h1>Generating new keys...</h1>
      </q-logo-block>
    `
  }

  renderGenerated() {
    return html`
      <q-logo-block>
        <h1>Keys generated</h1>
        <a class="download" download="qqself_keys.txt" href="${this.createDownloadLink()}"
          >Download key file</a
        >
        <q-button class="login" @clicked="${this.onLogin.bind(this)}">Continue to login</q-button>
      </q-logo-block>
    `
  }

  render() {
    if (this.cryptor == null && !this.generating) {
      return this.renderRegister()
    } else if (this.generating) {
      return this.renderGenerating()
    } else {
      return this.renderGenerated()
    }
  }
}
