import "./logo"

import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"

import { colors, font } from "../styles"

declare global {
  interface HTMLElementTagNameMap {
    "q-panel": Panel
  }
}

@customElement("q-panel")
export class Panel extends LitElement {
  static styles = [
    font,
    css`
      .root {
        background-color: ${colors.background.dark};
        border: 1px solid ${colors.highlight.dark};
      }
      .root .title {
        text-align: center;
        text-transform: uppercase;
        font-weight: bold;
        line-height: 2;
        border-bottom: 1px solid ${colors.highlight.dark};
        color: ${colors.highlight.dark};
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
