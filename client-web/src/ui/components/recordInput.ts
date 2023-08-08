import "../controls/logo"

import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"

import { UiRecord } from "../../../bridge/pkg"

export type RecordSaveEvent = CustomEvent<{ record: UiRecord }>
export type RecordUpdateEvent = CustomEvent<{ input: string }>

declare global {
  interface HTMLElementTagNameMap {
    "q-record-input": RecordInput
  }
}

@customElement("q-record-input")
export class RecordInput extends LitElement {
  @property({ type: Object })
  initialRecord?: UiRecord

  @property({ type: String })
  input = ""

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
    const record: UiRecord = UiRecord.parse(this.input, true) as never // Save button enabled only when record was created, so cast is safe
    const event: RecordSaveEvent = new CustomEvent("save", { detail: { record: record } })
    this.dispatchEvent(event)
  }

  entryUpdated(event: InputEvent) {
    this.input = (event.target as HTMLInputElement).value
    const updateEvent: RecordUpdateEvent = new CustomEvent("update", {
      detail: {
        input: this.input,
      },
    })
    this.dispatchEvent(updateEvent)
  }

  validate() {
    try {
      UiRecord.parse(this.input, true)
      return null
    } catch (ex) {
      return String(ex)
    }
  }

  render() {
    const error = this.validate()
    const isEntryValid = error == null
    return html`<div class="root">
      <div class="input">
        <input placeholder="New entry to add" class="text" type="text" .value="${
          this.input
        }" @input=${this.entryUpdated.bind(this)}></input>
        <button ?disabled=${!isEntryValid} @click="${this.onSave.bind(this)}">Save</button>  
      </div>
      <div class="error">${error}</div>
    </div>`
  }
}
