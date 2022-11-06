import { css, html, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";
import { Keys } from "../../core/pkg/qqself_client_web_core";
import "../components/logoBlock";
import "../controls/button";

declare global {
  interface HTMLElementTagNameMap {
    "q-progress-page": ProgressPage;
  }
}

@customElement("q-progress-page")
export class ProgressPage extends LitElement {
  @property({ type: Object })
  keys: Keys | null = null;

  static styles = css``;

  render() {
    return html`
      <q-logo-block>
        <h1>Progress</h1>
        <p>
          Here goes some statistics about progress, skills charts, heroes, roles
          and whatever else, I just need some text to render.
        </p>
      </q-logo-block>
    `;
  }
}
