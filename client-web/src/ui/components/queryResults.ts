import "../controls/logo"
import "../controls/button"
import "./entryInput"

import { css, html, LitElement, PropertyValues } from "lit"
import { customElement, property, query, state } from "lit/decorators.js"

import { validateQuery } from "../../../bridge/pkg"
import { colors } from "../styles"
import { EntrySaveEvent, EntryUpdateEvent } from "./entryInput"

declare global {
  interface HTMLElementTagNameMap {
    "q-query-results": QueryResults
  }
}

export type QueryUpdatedEvent = CustomEvent<{ query: string }>

@customElement("q-query-results")
export class QueryResults extends LitElement {
  @property({ type: Object })
  data: Record<string, string[]> = {}

  @property()
  query = ""

  @state()
  currentQuery = ""

  @state()
  queryValidationError: string | undefined = undefined

  @query(".query")
  queryElement!: HTMLInputElement

  @query(".results")
  resultsElement!: HTMLElement

  @state()
  entry = ""

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
    .queryResults .results .day {
      margin-top: 15px;
      color: ${colors.highlight.dark};
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
    .entries .result .edit {
      display: none;
    }
    .entries .result:hover .edit {
      display: block;
    }
  `

  firstUpdated() {
    this.queryElement.value = this.query
    // Force input event handler to run
    this.queryElement.dispatchEvent(new Event("input"))
  }

  onSave(e: EntrySaveEvent) {
    const event: EntrySaveEvent = new CustomEvent("save", {
      detail: {
        entry: e.detail.entry,
      },
    })
    this.dispatchEvent(event)
    this.entry = ""
  }

  onEntryUpdated(e: EntryUpdateEvent) {
    this.entry = e.detail.entry
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

  onEditClicked(entry: string) {
    this.entry = entry
  }

  renderDay(day: string, entry: string[]) {
    return html`
      <div>
        <div class="day">${day}</div>
        <div class="entries">
          ${entry.map(
            (v) =>
              html`<div class="result">
                <div class="text">${v}</div>
                <q-button
                  class="edit"
                  @clicked=${this.onEditClicked.bind(this, `${day} ${v}`)}
                  icon="edit"
                ></q-button>
              </div>`,
          )}
        </div>
      </div>
    `
  }

  render() {
    return html`<div class="queryResults">
      <input placeholder="Query to filter the data" class="query" .value="${
        this.query
      }" @input=${this.onQueryUpdated.bind(this)}></input>
      <div class="error">${this.queryValidationError}</div>
      <div class="results">
        ${Object.entries(this.data).map(([day, entry]) => this.renderDay(day, entry))}
      </div>
      <q-entry-input class="newEntry" .entry=${this.entry} @save=${this.onSave.bind(
        this,
      )} @update=${this.onEntryUpdated.bind(this)}></q-entry-input>
    </div>`
  }

  updated(changedProperties: PropertyValues) {
    // If data got updated, then automatically scroll results to the bottom, to the most recent entry
    if (changedProperties.has("data")) {
      this.resultsElement.scrollTop = this.resultsElement.scrollHeight
    }
  }
}
