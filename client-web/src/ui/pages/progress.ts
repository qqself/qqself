import { css, html, LitElement } from "lit"
import { customElement, property, state } from "lit/decorators.js"
import "../components/logoBlock"
import "../controls/button"
import "../controls/notification"
import "../components/queryResults"
import "../components/skills"
import "../components/statusBar"

import { Store } from "../../app/store"
import { EntrySaveEvent } from "../components/entryInput"
import { DateDay, QueryResultEntry, SkillData } from "../../../bridge/pkg"
import { QueryUpdatedEvent } from "../components/queryResults"

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
  queryResultsData: Record<string, string[]> = {}

  @state()
  skillsData: SkillData[] = []

  @state()
  error = ""

  @state()
  notifications: string[] = []

  @state()
  status: { status: "pending" | "completed"; op: string | null } = { status: "completed", op: null }

  static styles = css`
    .root {
      display: flex;
      margin: 10px;
    }
    .query-results {
      display: flex;
      flex-direction: column;
      flex-basis: 100%;
      margin-right: 15px;
    }
    .skills {
      display: flex;
      flex-direction: column;
      flex-basis: 30%;
    }
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

  async onQueryUpdated(e: QueryUpdatedEvent) {
    await this.store.dispatch("views.queryResults.queryUpdated", { query: e.detail.query })
    this.updateQueryResults()
  }

  onEntryAdded(e: EntrySaveEvent) {
    return this.store.dispatch("data.entry.added", { entry: e.detail.entry, callSyncAfter: true })
  }

  updateQueryResults() {
    this.queryResultsData = this.store.userState.views
      .query_results()
      .reduce<Record<string, string[]>>((acc, cur) => {
        const day = cur.day
        if (day in acc) {
          acc[day].push(cur.text)
        } else {
          acc[day] = [cur.text]
        }
        return acc
      }, {})
  }

  updateSkills() {
    this.skillsData = this.store.userState.views.view_skills()
  }

  connectedCallback() {
    super.connectedCallback()
    this.store.subscribe("views.update.queryResults", () => {
      this.updateQueryResults()
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
      (e) => (this.status = { ...this.status, op: e.operation }),
    )
    this.updateQueryResults()
    this.updateSkills()
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
                </div>`,
            )}
      </div>
    `
  }

  render() {
    return html`
      <div class="root">
        <q-query-results
          class="query-results"
          .data=${this.queryResultsData}
          .query=${`filter before=${DateDay.fromDate(new Date()).remove_days(30).toString()}. `}
          @queryUpdated=${this.onQueryUpdated.bind(this)}
          @save=${this.onEntryAdded.bind(this)}
        ></q-query-results>
        <q-skills class="skills" .skills=${this.skillsData}></q-skills>
      </div>
      ${this.error && html`<p>Error ${this.error}</p>`} ${this.renderNotifications()}
      <q-status-bar
        class="status"
        .status=${this.status.status}
        .currentOp=${this.status.op}
      ></q-status-bar>
    `
  }
}
