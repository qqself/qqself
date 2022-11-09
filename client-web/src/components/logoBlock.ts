import { css, html, LitElement } from "lit"
import { customElement } from "lit/decorators.js"
import "../controls/logo"

declare global {
  interface HTMLElementTagNameMap {
    "q-logo-block": LogoBlock
  }
}

@customElement("q-logo-block")
export class LogoBlock extends LitElement {
  static styles = css`
    .root {
      display: flex;
      justify-content: center;
      margin: 20px;
      height: 100%;
    }
    .content {
      text-align: center;
    }
  `

  render() {
    return html`<div class="root">
      <div class="content">
        <q-logo></q-logo>
        <slot></slot>
      </div>
    </div>`
  }
}
