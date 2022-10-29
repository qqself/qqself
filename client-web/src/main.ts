import { html, LitElement } from "lit";
import { customElement, state } from "lit/decorators.js";

import "./pages/loading";
import "./pages/register";
import "./pages/login";
import "./pages/enter";
import "./pages/progress";
import { Keys } from "./keyFile";

type Page = "enter" | "login" | "register" | "progress";

// Main global state, similar to the Redux
interface State {
  initComplete: boolean;
  page: Page;
  keys: Keys | null;
}

const defaultState: State = {
  initComplete: false,
  page: "enter",
  keys: null,
};

@customElement("q-main")
export class Main extends LitElement {
  @state()
  state = defaultState;

  constructor() {
    super();
    const availablePages = ["enter", "login", "register", "progress"];
    const page = window.location.hash.slice(1);
    if (availablePages.includes(page)) {
      this.state.page = page as Page;
    }
  }

  moveToPage(page: Page) {
    window.history.pushState(null, "", "#" + page);
    this.state = { ...this.state, page };
  }

  render() {
    if (!this.state.initComplete) {
      return html`<q-loading-page
        @loaded=${() => (this.state = { ...this.state, initComplete: true })}
      />`;
    }
    switch (this.state.page) {
      case "enter": {
        return html`<q-enter-page
          @login=${() => this.moveToPage("login")}
          @register=${() => this.moveToPage("register")}
        />`;
      }
      case "register": {
        return html`<q-register-page
          @registered=${(sender: any) => {
            const keys = sender.detail.keys as Keys;
            this.state = { ...this.state, keys };
            this.moveToPage("login");
          }}
        />`;
      }
      case "login": {
        return html`<q-login-page
          .keys=${this.state.keys}
          @loggedIn=${(sender: any) => {
            const keys = sender.detail.keys as Keys;
            this.state = { ...this.state, keys };
            this.moveToPage("progress");
          }}
        />`;
      }
      case "progress": {
        return html`<q-progress-page .keys=${this.state.keys} />`;
      }
    }
  }
}
