import { html, LitElement } from "lit"
import { customElement, property, state } from "lit/decorators.js"
import { AppJournalDay, DateDay } from "../../../bridge/pkg/qqself_client_web_bridge"
import "../components/logoBlock"
import "../controls/button"
import "../components/journal"
import "../components/skills"
import { Store } from "../../app/store"
import { EntrySaveEvent } from "../components/entryInput"

declare global {
  interface HTMLElementTagNameMap {
    "q-progress-page": ProgressPage
  }
}

@customElement("q-progress-page")
export class ProgressPage extends LitElement {
  @property({ type: Object })
  store!: Store

  @property({ type: Object })
  currentDay!: DateDay

  @state()
  journalData!: AppJournalDay

  @state()
  error = ""

  onSwitchDay(diff: number) {
    this.currentDay =
      diff > 0 ? this.journalData.day.add_days(1) : this.journalData.day.remove_days(1)
    this.updateJournal()
  }

  onEntryAdded(e: EntrySaveEvent) {
    return this.store.dispatch("data.entry.added", { entry: e.detail.entry, callSyncAfter: true })
  }

  updateJournal() {
    this.journalData = this.store.userState.views.journal_day(this.currentDay)
  }

  connectedCallback() {
    super.connectedCallback()
    this.store.subscribe("data.sync.succeeded", this.updateJournal.bind(this))
    this.updateJournal()
  }

  render() {
    return html`
      <q-logo-block>
        <h1>Progress</h1>
        <q-journal
          .data=${this.journalData}
          @next=${() => this.onSwitchDay(1)}
          @prev=${() => this.onSwitchDay(-1)}
          @save=${this.onEntryAdded.bind(this)}
        ></q-journal>
        <q-skills .data=${this.store.userState.views.view_skills().skills}></q-skills>
        ${this.error && html`<p>Error ${this.error}</p>`}
      </q-logo-block>
    `
  }
}
