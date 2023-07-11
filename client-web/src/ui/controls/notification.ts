import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"
import { defaultFont } from "../../constants"
import "./logo"
import "../controls/button"

declare global {
  interface HTMLElementTagNameMap {
    "q-notification": Notification
  }
}

@customElement("q-notification")
export class Notification extends LitElement {
  static styles = [
    defaultFont, // Safari is failing to get the font from the reset.css, repeat it here
    css`
      .root {
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: white;
        width: 100%;
        height: 100%;
        border: 2px solid black;
        padding: 10px;
      }
      .root .text {
        line-height: 3;
        font-weight: bold;
        font-size: 25px;
        text-align: center;
      }
    `,
  ]

  @property({ type: String })
  text!: string

  onClick() {
    this.dispatchEvent(new Event("clicked"))
  }

  render() {
    return html`<div class="root">
      <div class="text">
        ${this.text}
        <q-button @clicked=${this.onClick.bind(this)}>OK</q-button>
      </div>
    </div>`
  }
}
