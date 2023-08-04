import "../controls/logo"

import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"

import { validateEntry } from "../../../bridge/pkg"

export type EntrySaveEvent = CustomEvent<{ entry: string }>
export type EntryUpdateEvent = CustomEvent<{ entry: string }>

declare global {
  interface HTMLElementTagNameMap {
    "q-entry-input": EntryInput
  }
}

@customElement("q-entry-input")
export class EntryInput extends LitElement {
  @property()
  entry = ""

  static styles = css`
    .root .input {
      display: flex;
      justify-content: space-evenly;
    }
    .root .input .text {
      width: 100%;
      margin-right: 10px;
    }
    .root .error {
      color: red;
    }
  `

  onSave() {
    const event: EntrySaveEvent = new CustomEvent("save", {
      detail: {
        entry: this.entry,
      },
    })
    this.dispatchEvent(event)
    this.entry = ""
  }

  entryUpdated(event: InputEvent) {
    this.entry = (event.target as HTMLInputElement).value
    const updateEvent: EntryUpdateEvent = new CustomEvent("update", {
      detail: {
        entry: this.entry,
      },
    })
    this.dispatchEvent(updateEvent)
  }

  render() {
    const validationError = validateEntry(this.entry)
    const isEntryValid = validationError == undefined
    return html`<div class="root">
      <div class="input">
        <input placeholder="New entry to add" class="text" type="text" .value="${
          this.entry
        }" @input=${this.entryUpdated.bind(this)}></input>
        <button ?disabled=${!isEntryValid} @click="${this.onSave.bind(this)}">Save</button>  
      </div>
      <div class="error">${validationError}</div>
    </div>`
  }
}
