import "../controls/logo"
import "../controls/button"
import "./recordInput"

import { css, html, LitElement, PropertyValues } from "lit"
import { customElement, property, query, state } from "lit/decorators.js"

import { UiRecord, validateQuery } from "../../../qqself_core"
import { colors } from "../styles"
import { RecordSaveEvent, RecordUpdateEvent } from "./recordInput"

declare global {
  interface HTMLElementTagNameMap {
    "q-query-results": QueryResults
  }
}

export type QueryUpdatedEvent = CustomEvent<{ query: string }>

@customElement("q-query-results")
export class QueryResults extends LitElement {
  @property({ type: Object })
  data?: Record<string, UiRecord[]>

  @property()
  query?: string

  @query(".query")
  queryElement!: HTMLInputElement

  @query(".results")
  resultsElement!: HTMLElement

  @state()
  currentQuery = ""

  @state()
  queryValidationError?: string

  @state()
  currentRecord?: UiRecord

  @state()
  currentRecordString = ""

  static styles = css`
    .queryResults .query {
      box-sizing: border-box;
      width: 100%;
    }
    .queryResults {
      text-align: left;
      background-color: ${colors.background.dark};
      border: 1px solid ${colors.highlight.dark};
      padding: 15px;
    }
    .queryResults .error {
      color: red;
    }
    .queryResults .results .entries {
      background-color: ${colors.background.light};
      font-family: "Monaco", "Courier", "Courier New";
      padding: 10px;
    }
    .queryResults .results {
      margin: 10px 0;
      overflow: auto;
      max-height: 600px;
    }
    .entries .result {
      display: flex;
      justify-content: space-between;
    }
    .entries .result .text {
      white-space: pre;
    }
    .entries .result .edit,
    .entries .result .delete {
      display: none;
    }
    .entries .result:hover .edit,
    .entries .result:hover .delete {
      display: block;
    }
    .result-buttons {
      display: flex;
    }
  `

  firstUpdated() {
    this.queryElement.value = this.query ?? ""
    // Force input event handler to run
    this.queryElement.dispatchEvent(new Event("input"))
  }

  onSave(e: RecordSaveEvent) {
    const event: RecordSaveEvent = new CustomEvent("save", {
      detail: {
        record: e.detail.record,
      },
    })
    this.dispatchEvent(event)
    this.currentRecord = undefined
    this.currentRecordString = ""
  }

  onEntryUpdated(e: RecordUpdateEvent) {
    this.currentRecordString = e.detail.input
  }

  onQueryUpdated(sender: InputEvent) {
    this.query = (sender.target as HTMLInputElement).value
    this.queryValidationError = validateQuery(this.query)
    if (!this.queryValidationError) {
      const event: QueryUpdatedEvent = new CustomEvent("queryUpdated", {
        detail: {
          query: this.query,
        },
      })
      this.dispatchEvent(event)
    }
  }

  onEditClicked(record: UiRecord) {
    this.currentRecord = record
    this.currentRecordString = record.to_string(true, false)
  }

  onDeleteClicked(record: UiRecord) {
    const event: RecordSaveEvent = new CustomEvent("save", {
      detail: {
        record: record.created_deleted_record(),
      },
    })
    this.dispatchEvent(event)
  }

  renderDay(records: UiRecord[]) {
    // First entry of the day render with date prefix, others without a date, but with space offset
    // equal to date prefix length. We are using monospace font, so number of spaces is fine
    const dateLength = 11
    const renderRecord = (v: UiRecord, i: number) =>
      " ".repeat(i == 0 ? 0 : dateLength) + v.to_string(i == 0, false)
    return html`
      <div>
        <div class="entries">
          ${records.map(
            (v, i) =>
              html`<div class="result">
                <div class="text">${renderRecord(v, i)}</div>
                <div class="result-buttons">
                  <q-button
                    class="edit"
                    @clicked=${this.onEditClicked.bind(this, v)}
                    icon="edit"
                  ></q-button>
                  <q-button
                    class="delete"
                    @clicked=${this.onDeleteClicked.bind(this, v)}
                    icon="delete"
                  ></q-button>
                </div>
              </div>`,
          )}
        </div>
      </div>
    `
  }

  render() {
    return html`<div class="queryResults">
      <input placeholder="Query to filter the data" class="query" .value="${
        this.query ?? ""
      }" @input=${this.onQueryUpdated.bind(this)}></input>
      <div class="error">${this.queryValidationError}</div>
      <div class="results">
        ${Object.entries(this.data ?? {}).map(([, entry]) => this.renderDay(entry))}
      </div>
      <q-record-input class="newEntry" .initialRecord=${this.currentRecord} .input=${
        this.currentRecordString
      } @update=${this.onEntryUpdated.bind(this)}  @save=${this.onSave.bind(this)}></q-record-input>
    </div>`
  }

  updated(changedProperties: PropertyValues) {
    // If data got updated, then automatically scroll results to the bottom, to the most recent entry
    if (changedProperties.has("data")) {
      this.resultsElement.scrollTop = this.resultsElement.scrollHeight
    }
  }
}
