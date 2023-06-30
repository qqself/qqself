import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"
import "../controls/logo"

declare global {
  interface HTMLElementTagNameMap {
    "q-status-bar": StatusBar
  }
}

@customElement("q-status-bar")
export class StatusBar extends LitElement {
  @property()
  status: "pending" | "completed" = "completed"

  @property()
  currentOp: string | null = null

  static styles = css`
    .root {
      border: 1px solid gray;
      padding: 0 5px;
    }
  `

  render() {
    const status = this.status == "pending" ? "Sync pending" : "In sync"
    const op = this.currentOp ? `. ${this.currentOp}` : ""
    return html`<div class="root">${status}${op}</div>`
  }
}
