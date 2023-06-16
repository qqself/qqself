import { css, html, LitElement } from "lit"
import { customElement, property, state } from "lit/decorators.js"
import { Keys } from "../../../bridge/pkg/qqself_client_web_bridge"
import "../components/logoBlock"
import "../controls/button"
import { Store } from "../../app/store"
import * as Auth from "../../app/auth"

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
  generatedKeys: Keys | null = null

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
    this.generatedKeys = await Auth.newKeys()
    this.generating = false
  }

  createDownloadLink() {
    const keys = this.generatedKeys! // By that time keys always exists
    const blob = new Blob([keys.serialize()], { type: "text/plain" })
    return window.URL.createObjectURL(blob)
  }

  onLogin() {
    return this.store.dispatch("auth.registration.succeeded", { keys: this.generatedKeys! })
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
    if (this.generatedKeys == null && !this.generating) {
      return this.renderRegister()
    } else if (this.generating) {
      return this.renderGenerating()
    } else {
      return this.renderGenerated()
    }
  }
}
