import { css, html, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";
import "../controls/logo";

declare global {
  interface HTMLElementTagNameMap {
    "q-journal": Journal;
  }
}

@customElement("q-journal")
export class Journal extends LitElement {
  @property({ type: Array })
  entries: string[] = [];

  render() {
    const text = this.entries.join("\n");
    return html`<div class="journal">
      <h2>Entries</h2>
      ${text}
    </div>`;
  }
}
