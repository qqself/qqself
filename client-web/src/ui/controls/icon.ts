import "./logo"

import square from "@fortawesome/fontawesome-free/svgs/regular/square.svg"
import squareCheck from "@fortawesome/fontawesome-free/svgs/regular/square-check.svg"
import brain from "@fortawesome/fontawesome-free/svgs/solid/brain.svg"
import palette from "@fortawesome/fontawesome-free/svgs/solid/palette.svg"
import personRunning from "@fortawesome/fontawesome-free/svgs/solid/person-running.svg"
import squarePen from "@fortawesome/fontawesome-free/svgs/solid/square-pen.svg"
import squarePlus from "@fortawesome/fontawesome-free/svgs/solid/square-plus.svg"
import squareXmark from "@fortawesome/fontawesome-free/svgs/solid/square-xmark.svg"
import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"

declare global {
  interface HTMLElementTagNameMap {
    "q-icon": Icon
  }
}

export type IconName =
  | "activityIntelligent"
  | "activityPhysical"
  | "activityCreative"
  | "delete"
  | "add"
  | "edit"
  | "check"
  | "unchecked"
@customElement("q-icon")
export class Icon extends LitElement {
  @property({ type: String })
  name!: IconName

  static styles = css`
    .icon {
      width: var(--icon-size, 64px);
      height: var(--icon-size, 64px);
    }
  `

  render() {
    const icons: Record<IconName, unknown> = {
      activityCreative: palette,
      activityIntelligent: brain,
      activityPhysical: personRunning,
      delete: squareXmark,
      add: squarePlus,
      edit: squarePen,
      check: squareCheck,
      unchecked: square,
    }
    const icon = icons[this.name]
    if (icon) {
      return html`<div class="icon"><img src=${icon} /></div>`
    } else {
      throw new Error("Unexpected icon name:" + this.name)
    }
  }
}
