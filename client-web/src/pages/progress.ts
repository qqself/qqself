import { css, html, LitElement } from "lit";
import { customElement, property, state } from "lit/decorators.js";
import { API, Keys } from "../../core/pkg/qqself_client_web_core";
import { find } from "../api";
import "../components/logoBlock";
import "../controls/button";
import "../components/journal";
import { EncryptionPool } from "../encryptionPool";
import { log } from "../logger";

declare global {
  interface HTMLElementTagNameMap {
    "q-progress-page": ProgressPage;
  }
}

@customElement("q-progress-page")
export class ProgressPage extends LitElement {
  @property({ type: Object })
  keys: Keys | null = null;

  @property({ type: Object })
  encryptionPool: EncryptionPool | null = null;

  @state()
  entries: string[] = [];

  @state()
  error = "";

  async connectedCallback() {
    super.connectedCallback();
    try {
      const start = performance.now();
      const lines = await find(this.keys!);
      const requestFinished = performance.now();
      this.entries = await this.encryptionPool!.decryptAll(lines, this.keys!);
      const decrypted = performance.now();
      log(
        `Entries loaded in ${Math.floor(decrypted - start)}. API=${Math.floor(
          requestFinished - start
        )} Decryption=${Math.floor(decrypted - requestFinished)}`
      );
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
