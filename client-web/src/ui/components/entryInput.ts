import { css, html, LitElement } from "lit"
import { customElement, property, state } from "lit/decorators.js"
import { validateEntry } from "../../../bridge/pkg"
import "../controls/logo"

export type EntrySaveEvent = CustomEvent<{ entry: string }>

declare global {
  interface HTMLElementTagNameMap {
    "q-entry-input": EntryInput
  }
}

@customElement("q-entry-input")
export class EntryInput extends LitElement {
  @property()
  entry = ""

  @state()
  currentEntry = ""

  @state()
  isEntryValid = false

  @state()
  validationError: string | undefined = ""

  static styles = css`
    .root input {
      width: 300px;
    }
    .root .error {
      color: red;
    }
  `

  firstUpdated() {
    this.currentEntry = this.entry
    this.validateEntry()
  }

  onSave() {
    const event: EntrySaveEvent = new CustomEvent("save", {
      detail: {
        entry: this.currentEntry,
      },
    })
    this.dispatchEvent(event)

    this.currentEntry = ""
    this.validateEntry()
  }

  entryUpdated(event: InputEvent) {
    this.currentEntry = (event.target as HTMLInputElement).value
    this.validateEntry()
  }

  validateEntry() {
    this.validationError = validateEntry(this.currentEntry)
    this.isEntryValid = this.validationError == undefined
  }

  render() {
    return html`<div class="root">
      <input type="text" .value="${this.currentEntry}" @input=${this.entryUpdated.bind(
      this
    )}></input>
      <button ?disabled=${!this.isEntryValid} @click="${this.onSave.bind(this)}">Save</button>  
      <div class="error">${this.validationError}</div>
    </div>`
  }
}
