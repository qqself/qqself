import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"
import "./logo"
import brain from "@fortawesome/fontawesome-free/svgs/solid/brain.svg"
import personRunning from "@fortawesome/fontawesome-free/svgs/solid/person-running.svg"
import palette from "@fortawesome/fontawesome-free/svgs/solid/palette.svg"

declare global {
  interface HTMLElementTagNameMap {
    "q-icon": Icon
  }
}

@customElement("q-icon")
export class Icon extends LitElement {
  @property({ type: String })
  name!: "brain" | "person-running" | "palette"

  static styles = css`
    .icon {
      width: var(--icon-size, 64px);
      height: var(--icon-size, 64px);
    }
    .icon img {
      color: pink;
    }
  `

  render() {
    let icon = null

    if (this.name == "brain") {
      icon = brain as unknown
    } else if (this.name == "person-running") {
      icon = personRunning as unknown
    } else {
      icon = palette as unknown
    }

    if (icon) {
      return html`<div class="icon"><img src=${icon} /></div>`
    } else {
      throw new Error("Unexpected icon name:" + this.name)
    }
  }
}
