import { css, html, LitElement } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";
import "../components/logoBlock";
import "../controls/button";
import { Keys, stringToKeys } from "../keyFile";

declare global {
  interface HTMLElementTagNameMap {
    "q-login-page": LoginPage;
  }
}

// TODO Failed to support password managers for saving keys because of shadow root incompatibilities
//      Looks like one way would be put the login form out of the shadow root, see litElement.createRenderRoot
@customElement("q-login-page")
export class LoginPage extends LitElement {
  @property({ type: Object })
  keys: Keys | null = null;

  @query("#openFile")
  openFile: HTMLInputElement | undefined;

  @state()
  publicKey = "";

  @state()
  privateKey = "";

  @state()
  error = "";

  connectedCallback() {
    super.connectedCallback();
    if (this.keys) {
      this.publicKey = this.keys.publicKey;
      this.privateKey = this.keys.privateKey;
    }
  }

  keyFileOpened(e: Event) {
    const reader = new FileReader();
    reader.onload = (e: any) => {
      const keysResult = stringToKeys(e.target.result);
      if (keysResult instanceof Error) {
        this.error = keysResult.message;
      } else {
        this.publicKey = keysResult.publicKey;
        this.privateKey = keysResult.privateKey;
      }
    };
    reader.readAsText((e.target as any).files[0]);
  }

  login() {
    this.dispatchEvent(
      new CustomEvent("loggedIn", {
        detail: {
          keys: { publicKey: this.publicKey, privateKey: this.privateKey },
        },
      })
    );
  }

  resetForm() {
    this.publicKey = "";
    this.privateKey = "";
    this.error = "";
  }

  isLoginDisabled() {
    return !this.publicKey || !this.privateKey;
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
  `;

  render() {
    return html`
      <q-logo-block>
        <h1>Login</h1>
        <a
          class="keyFile"
          href=""
          @click="${(event: Event) => {
            this.resetForm();
            this.openFile?.click();
            event.preventDefault();
          }}"
          >Login using key file</a
        >
        <input id="openFile" type="file" @change="${this.keyFileOpened}" />
        <div class="keyInput">
          <label for="username">Public key</label>
          <input
            type="text"
            id="username"
            autocomplete="username"
            name="username"
            .value="${this.publicKey}"
            required
            @input="${(e: any) => (this.publicKey = e.target.value)}"
          />
        </div>
        <div class="keyInput">
          <label for="current-password">Private key</label>
          <input
            id="current-password"
            autocomplete="current-password"
            type="text"
            name="password"
            .value="${this.privateKey}"
            required
            @input="${(e: any) => (this.privateKey = e.target.value)}"
          />
        </div>
        <p class="error">${this.error}</p>
        <q-button
          class="login"
          @clicked="${this.login}"
          ?disabled=${this.isLoginDisabled()}
          ?isSubmit=${true}
          >Login</q-button
        >
      </q-logo-block>
    `;
  }
}
