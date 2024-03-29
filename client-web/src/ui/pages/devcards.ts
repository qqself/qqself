import "../components/skills"
import "../components/week"
import "../components/recordInput"
import "../components/queryResults"
import "../components/statusBar"
import "../components/postNote"
import "../controls/panel"
import "../controls/notification"
import "../controls/icon"
import "../pages/progress"
import "../pages/growth"
import "../pages/login"
import "../pages/register"

import { css, html, LitElement, TemplateResult } from "lit"
import { customElement, property, state } from "lit/decorators.js"

import { DateDay, SkillWeek, UiRecord } from "../../../qqself_core"
import { trace } from "../../logger"
import { OfflineApi, TestStore } from "../../utilsTests"
import { RecordSaveEvent } from "../components/recordInput"

declare global {
  interface HTMLElementTagNameMap {
    "q-devcards": DevcardsPage
  }
}

@customElement("q-card")
export class Card extends LitElement {
  @property()
  name = ""

  render() {
    const filter = window.location.hash.slice(1).split(":")
    if (filter.length == 2 && !this.name.includes(decodeURI(filter[1]))) {
      return // Card is filtered out
    }
    const renderTitle = filter.length != 2 // render title only when filter is not set and we are rendering many controls
    if (renderTitle) {
      return html`<div class="card">
        <h2
          .onclick=${() => {
            window.location.hash = `devcards:${this.name}`
            window.location.reload()
          }}
        >
          Card: ${this.name}
        </h2>
        <slot></slot>
      </div>`
    } else {
      return html`<slot></slot>`
    }
  }
}

// Custom page with all UI elements, used mostly for development, kinda like storybooks
@customElement("q-devcards-page")
export class DevcardsPage extends LitElement {
  store = new TestStore(undefined, new OfflineApi())

  @state()
  cards: TemplateResult | null = null

  static styles = css`
    .devcards {
      height: 100%;
    }
  `

  async connectedCallback() {
    super.connectedCallback()
    await this.store.dispatch("init.started", null)
    await this.store.dispatch("auth.registration.started", { mode: "automatic" })
    await this.configure()
  }

  async configure() {
    const input = `2022-07-15 00:00 00:02 qqself. skill kind=🧠. Entrepreneur 
2022-07-15 00:00 00:03 read. skill kind=🧠. Reader
2022-07-15 00:00 00:11 drums. skill kind=🫀. Drummer
2023-07-15 00:00 00:12 sculpture. skill kind=🫀. Sculptor
2022-11-09 09:20 11:00 qqself. Query for DynamoDB storage, figured out we should always include items equal to after_timestamp
2022-11-09 03:05 23:25 drums
2022-11-09 11:25 12:30 qqself. Completed DynamoDB storage, created a PR
2022-11-09 13:40 15:20 qqself. AWS config changes for Dynamo, switch to Dynamo storage in code
2022-11-09 15:50 16:50 qqself. Deploying DynamoDB changes, found a race condition. Nope, it was not
2022-11-09 17:15 17:40 qqself. Finished DynamoDB storage, created a PR
2022-11-09 21:40 23:15 read    
2022-11-10 09:00 09:50 drums. practice of melodics. Working on double pedal speed
2022-11-10 13:00 14:30 run distance=15 elevation=340 
2022-11-10 15:00 15:30 stretch
2022-11-10 19:00 21:00 sculpture of "David". Good progress overall, need to revisit posture
2022-11-11 12:30 13:10 qqself. API to return JournalDay, Rust Analyser issues with multiple targets in workspace
2022-11-11 16:30 17:00 qqself. Found an issue with date duration because of handcrafted dates, checked for alternatives
2022-11-11 18:00 20:00 qqself. Refactoring all custom date and times structs to \`time\` create
2022-11-11 21:30 23:30 qqself. Fixed all the tests, migrated fully to new date and time structures, created a PR`
    for (const entry of input.split("\n")) {
      await this.store.dispatch("data.entry.added", { entry, callSyncAfter: false })
    }

    const groupedEntries = this.store.userState.views
      .query_results()
      .reduce<Record<string, UiRecord[]>>((acc, cur) => {
        const day = cur.day()
        if (day in acc) {
          acc[day].push(cur)
        } else {
          acc[day] = [cur]
        }
        return acc
      }, {})

    const weekProgress = [
      { name: "Runner", progress: 20, target: 60 * 5 },
      { name: "Reader", progress: 60 * 5 + 25, target: 60 * 5 },
      { name: "Drummer", progress: 60, target: 60 },
      { name: "Writer", progress: 5.5 * 60, target: 60 * 7 },
      { name: "Entrepreneur", progress: 6.3 * 60, target: 60 * 10 },
      { name: "Athlete", progress: 140, target: 60 * 7 },
      { name: "Swimmer", progress: 0, target: 60 },
      { name: "Sculptor", progress: 0, target: 60 },
      { name: "Finnish speaker", progress: 0, target: 60 },
    ] as SkillWeek[]

    // Render all the devcards. If page hash ends with `/devcards:[CARD_NAME]` then only the card with such name will be rendered
    this.cards = html`<div class="devcards">
      <!-- Controls -->
      <q-card name="Button - Normal">
        <q-button>Normal</q-button>
        <q-button icon="edit"></q-button>
      </q-card>
      <q-card name="Button - Disabled">
        <q-button disabled>Normal</q-button>
        <q-button icon="add" disabled></q-button>
      </q-card>
      <q-card name="Panel">
        <q-panel title="Dev panel">
          <div>Content #1</div>
          <div>Content #2</div>
        </q-panel>
      </q-card>

      <q-card name="Notification">
        <q-notification text="Running skill level increased to 53"> </q-notification>
      </q-card>

      <q-card name="Icon">
        <q-icon name="activityIntelligent"></q-icon>
      </q-card>

      <!-- Components -->
      <q-card name="QueryResults">
        <q-query-results .data=${groupedEntries}></q-query-results>
      </q-card>

      <q-card name="Skills">
        <q-skills .skills=${this.store.userState.views.view_skills()}></q-skills>
      </q-card>

      <q-card name="WeekView">
        <q-week-view .data=${weekProgress}></q-week-view>
      </q-card>

      <q-card name="AddEntry - Valid">
        <q-record-input
          input="2022-11-09 11:25 12:30 qqself. Added entry input"
          @save=${(e: RecordSaveEvent) => trace(JSON.stringify(e.detail))}
        ></q-record-input>
      </q-card>

      <q-card name="AddEntry - Invalid">
        <q-record-input input="2022-11-09 11:25 12:30 foo. foo"></q-record-input>
      </q-card>

      <q-card name="AddEntry - Empty">
        <q-record-input></q-record-input>
      </q-card>

      <q-card name="Status bar - Default">
        <q-status-bar> </q-status-bar>
      </q-card>

      <q-card name="Status bar - Pending">
        <q-status-bar status="pending" currentOp="Fetching data..."> </q-status-bar>
      </q-card>

      <q-card name="Post Note">
        <q-post-note
          text="This is a text, about something very important. It should be rather long as very likely there would be much of text"
        ></q-post-note>
      </q-card>

      <!-- Pages -->
      <q-card name="Progress page">
        <q-progress-page
          .store="${this.store}"
          .currentDay=${DateDay.fromDate(new Date(2022, 10, 10))}
        ></q-progress-page>
      </q-card>

      <q-card name="Growth page">
        <q-growth-page .store="${this.store}"></q-growth-page>
      </q-card>

      <q-card name="Login page">
        <q-login-page .store="${this.store}"></q-login-page>
      </q-card>

      <q-card name="Register page">
        <q-register-page .store="${this.store}"></q-register-page>
      </q-card>
    </div>`
  }

  render() {
    return this.cards ?? html`Loading test data...`
  }
}
