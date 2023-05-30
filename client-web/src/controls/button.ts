import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"
import { classMap } from "lit/directives/class-map.js"
import { defaultFont } from "../constants"
import "../controls/logo"

declare global {
  interface HTMLElementTagNameMap {
    "q-button": Button
  }
}

@customElement("q-button")
export class Button extends LitElement {
  static styles = [
    defaultFont, // Safari is failing to get the font from the reset.css, repeat it here
    css`
      .root button {
        color: white;
        background-color: black;
      }
      .disabled button {
        background-color: #aaa;
        color: #fff;
      }
    `,
  ]

  @property({ type: Boolean })
  disabled = false

  @property({ type: Boolean })
  isSubmit = false

  onClick() {
    this.dispatchEvent(new Event("clicked"))
  }

  render() {
    const classes = { disabled: this.disabled, root: true }
    const buttonType = this.isSubmit ? "submit" : "button"
    return html`<div class=${classMap(classes)}>
      <button @click="${this.onClick}" type="${buttonType}">
        <slot></slot>
      </button>
    </div>`
  }
}
