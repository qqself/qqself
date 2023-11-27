import "../controls/logo"

import { css, html, LitElement, unsafeCSS } from "lit"
import { customElement, property, state } from "lit/decorators.js"

import font from "../static/caveat.ttf"

declare global {
  interface HTMLElementTagNameMap {
    "q-post-note": PostNote
  }
}

export const PostNoteSize = 300
const size = PostNoteSize - 30 // Remove margins

@customElement("q-post-note")
export class PostNote extends LitElement {
  @property()
  text?: string

  @state()
  rotationClass = ""

  // We expect to have many post notes on a screen and using dynamic styles may have performance
  // implications. So let's go with set of pre-generated CSS classes for rotations instead
  static styles = css`
    @font-face {
      font-family: "Caveat";
      src: url(${unsafeCSS(font)}) format("truetype-variations");
      font-weight: 400;
    }
    .root {
      background-color: #ffc;
      height: ${size}px;
      width: ${size}px;
      margin: 15px;
      padding: 15px;
      box-shadow: 5px 5px 7px rgba(33, 33, 33, 0.7);
      font-family: "Caveat", cursive;
      font-size: 25px;
      overflow: hidden;
    }
    .rotation-5 {
      transform: rotate(-5deg);
    }
    .rotation-4 {
      transform: rotate(-4deg);
    }
    .rotation-3 {
      transform: rotate(-3deg);
    }
    .rotation-2 {
      transform: rotate(-2deg);
    }
    .rotation-1 {
      transform: rotate(-1deg);
    }
    .rotation1 {
      transform: rotate(1deg);
    }
    .rotation2 {
      transform: rotate(2deg);
    }
    .rotation3 {
      transform: rotate(3deg);
    }
    .rotation4 {
      transform: rotate(4deg);
    }
    .rotation5 {
      transform: rotate(5deg);
    }
  `

  firstUpdated() {
    const maxRotation = 5
    const minRotation = -5
    const rotation = Math.round(Math.random() * (maxRotation - minRotation) + minRotation)
    this.rotationClass = `rotation${rotation}`
  }

  render() {
    return html`<div class="root ${this.rotationClass}">
      <p>${this.text}</p>
    </div>`
  }
}
