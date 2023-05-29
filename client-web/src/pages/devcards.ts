import { html, LitElement, TemplateResult } from "lit"
import { customElement, property, state } from "lit/decorators.js"
import { App, DateDay, Keys } from "../../core/pkg"
import { EncryptionPool } from "../encryptionPool"
import "../components/skills"
import "../controls/panel"

declare global {
  interface HTMLElementTagNameMap {
    "q-devcards": DevcardsPage
  }
}

@customElement("q-card")
class Card extends LitElement {
  @property()
  name = ""
  render() {
    const filter = window.location.hash.slice(1).split(":")
    if (filter.length == 2 && decodeURI(filter[1]) != this.name) {
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
  @property({ type: Object })
  encryptionPool: EncryptionPool | null = null

  @state()
  cards: TemplateResult | null = null

  async connectedCallback() {
    super.connectedCallback()

    // Test data
    const testKeys: Keys = Keys.createNewKeys()
    const testApp = App.new(testKeys)
    const input = `2022-07-15 00:00 00:02 qqself. skill kind=🧠. Entrepreneur 
2022-07-15 00:00 00:03 read. skill kind=🧠. Reader
2022-07-15 00:00 00:11 drums. skill kind=🫀. Drummer
2022-11-09 09:20 11:00 qqself. Query for DynamoDB storage, figured out we should always include items equal to after_timestamp
2022-11-09 11:05 11:25 drums
2022-11-09 11:25 12:30 qqself. Completed DynamoDB storage, created a PR
2022-11-09 13:40 15:20 qqself. AWS config changes for Dynamo, switch to Dynamo storage in code
2022-11-09 15:50 16:50 qqself. Deploying DynamoDB changes, found a race condition. Nope, it was not
2022-11-09 17:15 17:40 qqself. Finished DynamoDB storage, created a PR
2022-11-09 21:40 23:15 read    
2022-11-10 09:00 09:50 qqself. Starting working on exposing DB to client-web
2022-11-10 10:00 10:20 drums
2022-11-10 10:30 11:05 qqself. JournalView with JournalDays, probably we don't need to JSON the whole journal, but a few days only
2022-11-10 12:30 13:10 qqself. API to return JournalDay, Rust Analyser issues with multiple targets in workspace
2022-11-10 16:30 17:00 qqself. Found an issue with date duration because of handcrafted dates, checked for alternatives
2022-11-10 18:00 20:00 qqself. Refactoring all custom date and times structs to \`time\` create
2022-11-10 21:30 23:30 qqself. Fixed all the tests, migrated fully to new date and time structures, created a PR`
    for (const entry of input.split("\n")) {
      testApp.add_entry(entry)
    }

    // Render all the devcards. If page hash ends with `/devcards:[CARD_NAME]` then only the card with such name will be rendered
    this.cards = html`<div class="devcards">
      <!-- Controls -->
      <q-card name="Panel">
        <q-panel title="Dev panel">
          <div>Content #1</div>
          <div>Content #2</div>
        </q-pane>
      </q-card>
      
      <!-- Components -->
      <q-card name="Journal">
        <q-journal
          .data=${testApp.journal_day(DateDay.fromDate(new Date(2022, 10, 10)))}
        ></q-journal>
      </q-card>

      <q-card name="Skills">
        <q-skills .data=${testApp.view_skills().skills}></q-skills>
      </q-card>

      <!-- Pages -->
      <q-card name="Progress page">
        <q-progress-page
          .keys="${testKeys}"
          .app=${testApp}
          .encryptionPool=${this.encryptionPool}
          .today=${new Date(2022, 10, 10)}
        ></q-progress-page>
      </q-card>
    </div>`
  }

  render() {
    return this.cards || html`Loading test data...`
  }
}
