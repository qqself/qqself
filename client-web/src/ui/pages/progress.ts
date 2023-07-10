import { css, html, LitElement } from "lit"
import { customElement, property, state } from "lit/decorators.js"
import { AppJournalDay, DateDay } from "../../../bridge/pkg"
import "../components/logoBlock"
import "../controls/button"
import "../controls/notification"
import "../components/journal"
import "../components/skills"
import "../components/statusBar"

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
  skillsData = ""

  @state()
  error = ""

  @state()
  notifications: string[] = []

  @state()
  status: { status: "pending" | "completed"; op: string | null } = { status: "completed", op: null }

  static styles = css`
    .status {
      position: fixed;
      bottom: 10px;
      right: 10px;
    }
    .notification {
      position: fixed;
      width: 50%;
      height: 50%;
      top: 25%;
      left: 25%;
    }
  `

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

  updateSkills() {
    this.skillsData = this.store.userState.views.view_skills().skills
  }

  connectedCallback() {
    super.connectedCallback()
    this.store.subscribe("views.update.journal", (event) => {
      if (event.update.day == this.currentDay.toString()) {
        // Update and rerender only if update day is the current one
        this.updateJournal()
      }
    })
    this.store.subscribe("views.update.skills", () => {
      this.updateSkills()
    })
    this.store.subscribe("views.notification.skills", (notification) => {
      this.notifications = [...this.notifications, notification.update.message]
    })
    this.store.subscribe("status.sync", (e) => (this.status = { ...this.status, status: e.status }))
    this.store.subscribe(
      "status.currentOperation",
      (e) => (this.status = { ...this.status, op: e.operation })
    )
    this.updateJournal()
    return this.store.dispatch("data.sync.init", null)
  }

  onNotificationDismiss(dismissed: string) {
    this.notifications = this.notifications.filter((v) => v != dismissed)
  }

  renderNotifications() {
    if (!this.notifications.length) return
    return html`
            ${this.notifications.map(
              (v) =>
                html`<div class="notification">
                  <q-notification
                    text=${v}
                    @clicked=${this.onNotificationDismiss.bind(this, v)}
                  ></q-notification>
                </div>`
            )}
      </div>
    `
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
        <q-skills .data=${this.skillsData}></q-skills>
        ${this.error && html`<p>Error ${this.error}</p>`} ${this.renderNotifications()}
        <q-status-bar
          class="status"
          .status=${this.status.status}
          .currentOp=${this.status.op}
        ></q-status-bar>
      </q-logo-block>
    `
  }
}
