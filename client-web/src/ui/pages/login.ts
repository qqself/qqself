import "../components/logoBlock"
import "../controls/button"

import { css, html, LitElement } from "lit"
import { customElement, property, query, state } from "lit/decorators.js"

import { Keys } from "../../../bridge/pkg/qqself_client_web_bridge"
import { Store } from "../../app/store"

declare global {
  interface HTMLElementTagNameMap {
    "q-login-page": LoginPage
  }
}

export type LoggedInEvent = CustomEvent<{ keys: Keys }>

// TODO Failed to support password managers for saving keys because of shadow root incompatibilities
//      Looks like one way would be put the login form out of the shadow root, see litElement.createRenderRoot
@customElement("q-login-page")
export class LoginPage extends LitElement {
  @property({ type: Object })
  store!: Store

  @query("#openFile")
  openFile: HTMLInputElement | undefined

  @state()
  error = ""

  firstUpdated() {
    this.store.subscribe("auth.login.errored", (args) => (this.error = args.error.message))
  }

  keyFileOpened(e: Event) {
    const reader = new FileReader()
    reader.onload = async (e: ProgressEvent<FileReader>) => {
      try {
        if (typeof e.target?.result != "string") {
          throw new Error("Failed to read a file")
        }
        await this.store.dispatch("auth.login.started", { keysString: e.target.result })
      } catch (ex) {
        this.error = String(ex)
      }
    }
    const input = e.target as HTMLInputElement
    if (input.files) {
      reader.readAsText(input.files[0])
    }
  }

  static styles = css`
    .keyInput {
      text-align: right;
      margin-bottom: 10px;
      width: 100%;
    }
    .keyInput input {
      font-size: 18px;
    }
    .keyFile {
      display: block;
      margin: 20px 0;
    }
    .login {
      display: block;
      margin-top: 20px;
    }
    #openFile {
      display: none;
    }
  `

  resetState() {
    this.error = ""
  }

  login() {
    this.resetState()
    this.openFile?.click()
  }

  register() {
    this.resetState()
    return this.store.dispatch("auth.registration.started", { mode: "interactive" })
  }

  render() {
    return html`
      <q-logo-block>
        <h1>Login</h1>
        <p>${this.error}</p>
        <input id="openFile" type="file" @change="${this.keyFileOpened.bind(this)}" />
        <div class="root">
          <q-button class="btn" @clicked="${this.login.bind(this)}">Login with key file</q-button>
          <q-button class="btn" @clicked="${this.register.bind(this)}">Register</q-button>
        </div>
      </q-logo-block>
    `
  }
}
