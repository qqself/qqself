import { css, html, LitElement, PropertyValues } from "lit"
import { customElement, property, state, query } from "lit/decorators.js"
import "../controls/logo"
import "./entryInput"
import { EntrySaveEvent } from "./entryInput"
import { colors } from "../styles"
import { validateQuery } from "../../../bridge/pkg"

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
      margin-top: 10px;
      overflow: auto;
      max-height: 600px;
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

  renderDay(day: string, entry: string[]) {
    return html`
      <div>
        <div class="day">${day}</div>
        <div class="entries">${entry.map((v) => html`<div class="result">${v}</div>`)}</div>
      </div>
    `
  }

  render() {
    return html`<div class="queryResults">
      <input class="query" .value="${this.query}" @input=${this.onQueryUpdated.bind(this)}></input>
      <div class="error">${this.queryValidationError}</div>
      <div class="results">
        ${Object.entries(this.data).map(([day, entry]) => this.renderDay(day, entry))}
      </div>
      <q-entry-input @save=${this.onSave.bind(this)}></q-entry-input>
    </div>`
  }

  updated(changedProperties: PropertyValues) {
    // If data got updated, then automatically scroll results to the bottom, to the most recent entry
    if (changedProperties.has("data")) {
      this.resultsElement.scrollTop = this.resultsElement.scrollHeight
    }
  }
}
