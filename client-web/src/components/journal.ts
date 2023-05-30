import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"
import { AppJournalDay } from "../../bridge/pkg/qqself_client_web_bridge"
import "../controls/logo"

declare global {
  interface HTMLElementTagNameMap {
    "q-journal": Journal
  }
}

@customElement("q-journal")
export class Journal extends LitElement {
  @property({ type: Object })
  data: AppJournalDay | null = null

  static styles = css`
    .journal h2 {
      text-align: center;
    }
    .journal {
      text-align: left;
    }
  `

  onNext(e: Event) {
    this.dispatchEvent(new Event("next"))
    e.preventDefault()
  }

  onPrev(e: Event) {
    this.dispatchEvent(new Event("prev"))
    e.preventDefault()
  }

  render() {
    if (!this.data) {
      return html`Loading data...`
    }
    const day = this.data.day
    const entries = this.data.entries.split("\n")
    return html`<div class="journal">
      <h2>
        <a href="#" @click=${this.onPrev}>⏴</a>
        ${day.toString()}
        <a href="#" @click=${this.onNext}>⏵</a>
      </h2>
      ${entries.map((v) => html`<p>${v}</p>`)}
    </div>`
  }
}
