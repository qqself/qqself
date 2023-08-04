import "./logo"

import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"
import { classMap } from "lit/directives/class-map.js"

import { font } from "../styles"
import { IconName } from "./icon"

declare global {
  interface HTMLElementTagNameMap {
    "q-button": Button
  }
}

@customElement("q-button")
export class Button extends LitElement {
  static styles = [
    font,
    css`
      .root button {
        color: white;
      }
      .root:not(.iconButton) button {
        background-color: black;
      }
      .root:not(.iconButton).disabled button {
        background-color: #aaa;
        color: #fff;
      }
      .root.iconButton.disabled button {
        background-color: #aaa;
      }
      .q-icon {
        --icon-size: 16px;
        margin-top: -3px;
        margin-bottom: 3px;
        display: block;
      }
      .iconButton button {
        padding: 0px;
        margin: 0px;
      }
    `,
  ]

  @property({ type: Boolean })
  disabled = false

  @property({ type: Boolean })
  isSubmit = false

  @property({ type: String })
  icon: IconName | null = null

  onClick() {
    this.dispatchEvent(new Event("clicked"))
  }

  render() {
    const classes = { disabled: this.disabled, root: true, iconButton: this.icon != null }
    const buttonType = this.isSubmit ? "submit" : "button"
    const content = this.icon
      ? html`<q-icon class="q-icon" name=${this.icon}></q-icon>`
      : html`<slot></slot>`
    return html`<div class=${classMap(classes)}>
      <button @click="${this.onClick.bind(this)}" type="${buttonType}">${content}</button>
    </div>`
  }
}
