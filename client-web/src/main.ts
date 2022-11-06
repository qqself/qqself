import { html, LitElement } from "lit";
import { customElement, state } from "lit/decorators.js";

import "./pages/loading";
import "./pages/register";
import "./pages/login";
import "./pages/progress";
import { Keys } from "../core/pkg/qqself_client_web_core";

type Page = "login" | "register" | "progress";

interface State {
  initComplete: boolean;
  page: Page;
  keys: Keys | null;
}

const defaultState: State = {
  initComplete: false,
  page: "login",
  keys: null,
};

@customElement("q-main")
export class Main extends LitElement {
  @state()
  state = defaultState;

  constructor() {
    super();
    const availablePages = ["login", "register", "progress"];
    const page = window.location.hash.slice(1);
    if (availablePages.includes(page)) {
      if (page != "progress" || this.state.keys) {
        this.state.page = page as Page; // Show progress only when keys are available
      } else {
        this.moveToPage("login");
      }
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
      case "login": {
        return html`<q-login-page
          .keys=${this.state.keys}
          @loggedIn=${(sender: any) => {
            const keys = sender.detail.keys as Keys;
            this.state = { ...this.state, keys };
            this.moveToPage("progress");
          }}
          @register=${() => this.moveToPage("register")}
        />`;
      }
      case "register": {
        return html`<q-register-page
          @registered=${() => this.moveToPage("login")}
        />`;
      }
      case "progress": {
        return html`<q-progress-page .keys=${this.state.keys} />`;
      }
    }
  }
}
