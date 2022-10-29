import { css, html, LitElement } from "lit";
import { customElement } from "lit/decorators.js";
import "../components/logoBlock";
import "../controls/button";

declare global {
  interface HTMLElementTagNameMap {
    "q-enter-page": EnterPage;
  }
}

@customElement("q-enter-page")
export class EnterPage extends LitElement {
  static styles = css`
    .root {
      padding-top: 20px;
    }
    .btn {
      display: block;
      margin-top: 10px;
    }
  `;

  selected(selection: "login" | "register") {
    return () => this.dispatchEvent(new Event(selection));
  }

  render() {
    return html`
      <q-logo-block>
        <div class="root">
          <q-button class="btn" @clicked="${this.selected("register")}"
            >Register</q-button
          >
          <q-button class="btn" @clicked="${this.selected("login")}"
            >Login</q-button
          >
        </div>
      </q-logo-block>
    `;
  }
}
