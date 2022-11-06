import { css, html, LitElement } from "lit";
import { customElement, property, state } from "lit/decorators.js";
import { API, Keys } from "../../core/pkg/qqself_client_web_core";
import { find } from "../api";
import "../components/logoBlock";
import "../controls/button";
import "../components/journal";

declare global {
  interface HTMLElementTagNameMap {
    "q-progress-page": ProgressPage;
  }
}

@customElement("q-progress-page")
export class ProgressPage extends LitElement {
  @property({ type: Object })
  keys: Keys | null = null;

  @state()
  entries: string[] = [];

  @state()
  error = "";

  async connectedCallback() {
    super.connectedCallback();
    if (!this.keys) {
      this.error =
        "Cannot fetch the data as keys are missing. Please reload the app and login again";
      return;
    }
    try {
      this.entries = await find(this.keys);
    } catch (ex: any) {
      this.error = ex as any;
    }
  }

  static styles = css``;

  render() {
    return html`
      <q-logo-block>
        <h1>Progress</h1>        
        <q-journal .entries=${this.entries}></q-journal>
        ${this.error && html`<p>Error ${this.error}</p>`}
      </q-logo-block>
    `;
  }
}
