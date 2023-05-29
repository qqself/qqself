import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"
import { defaultFont } from "../constants"
import "../controls/logo"

declare global {
  interface HTMLElementTagNameMap {
    "q-panel": Panel
  }
}

@customElement("q-panel")
export class Panel extends LitElement {
  static styles = [
    defaultFont, // Safari is failing to get the font from the reset.css, repeat it here
    css`
      .root {
        border: 1px solid black;
      }
      .root .title {
        text-align: center;
        line-height: 2;
        border-bottom: 1px solid black;
      }
      .root .content {
        margin: 10px;
      }
    `,
  ]

  @property({ type: String })
  title!: string

  render() {
    return html`<div class="root">
      <div class="title">${this.title}</div>
      <div class="content"><slot></slot></div>
    </div>`
  }
}
