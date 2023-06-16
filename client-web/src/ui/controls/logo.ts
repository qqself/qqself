import { css, html, LitElement } from "lit"
import { customElement } from "lit/decorators.js"
import logo from "./logo_512.png"

declare global {
  interface HTMLElementTagNameMap {
    "q-logo": Logo
  }
}

@customElement("q-logo")
export class Logo extends LitElement {
  static styles = css`
    .logo {
      max-width: 256px;
      max-height: 256px;
    }
  `

  render() {
    return html`<a href="/"><img class="logo" alt="logo" src=${logo} /></a>`
  }
}
