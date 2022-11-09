import { css, html, LitElement } from "lit"
import { customElement, property, query, state } from "lit/decorators.js"
import { Keys } from "../../core/pkg/qqself_client_web_core"
import "../components/logoBlock"
import "../controls/button"
import { EncryptionPool } from "../encryptionPool"

declare global {
  interface HTMLElementTagNameMap {
    "q-login-page": LoginPage
  }
}

// TODO Failed to support password managers for saving keys because of shadow root incompatibilities
//      Looks like one way would be put the login form out of the shadow root, see litElement.createRenderRoot
@customElement("q-login-page")
export class LoginPage extends LitElement {
  @property({ type: Object })
  keys: Keys | null = null

  @property({ type: Object })
  encryptionPool: EncryptionPool | null = null

  @query("#openFile")
  openFile: HTMLInputElement | undefined

  @state()
  error = ""

  keyFileOpened(e: Event) {
    const reader = new FileReader()
    reader.onload = (e: any) => {
      try {
        const keys = Keys.deserialize(e.target.result)
        this.dispatchEvent(
          new CustomEvent("loggedIn", {
            detail: {
              keys,
            },
          })
        )
      } catch (ex: any) {
        this.error = ex
      }
    }
    reader.readAsText((e.target as any).files[0])
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
    this.dispatchEvent(new Event("register"))
  }

  render() {
    return html`
      <q-logo-block>
        <h1>Login</h1>
        <p>${this.error}</p>
        <input id="openFile" type="file" @change="${this.keyFileOpened}" />
        <div class="root">
          <q-button class="btn" @clicked="${this.login}">Login with key file</q-button>
          <q-button class="btn" @clicked="${this.register}">Register</q-button>
        </div>
      </q-logo-block>
    `
  }
}
