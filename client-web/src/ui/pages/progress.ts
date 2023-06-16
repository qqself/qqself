import { html, LitElement } from "lit"
import { customElement, property, state } from "lit/decorators.js"
import { App, AppJournalDay, DateDay, Keys } from "../../../bridge/pkg/qqself_client_web_bridge"
import { find } from "../../app/api"
import "../components/logoBlock"
import "../controls/button"
import "../components/journal"
import "../components/skills"
import { EncryptionPool } from "../../app/encryptionPool/pool"
import { info } from "../../logger"
import { Storage } from "../../app/storage/storage"

declare global {
  interface HTMLElementTagNameMap {
    "q-progress-page": ProgressPage
  }
}

@customElement("q-progress-page")
export class ProgressPage extends LitElement {
  @property({ type: Object })
  // TODO Keys have to move to app
  keys: Keys | null = null

  @property({ type: Object })
  today: Date = new Date()

  @property({ type: Object })
  app: App | null = null

  @property({ type: Object })
  encryptionPool: EncryptionPool | null = null

  @state()
  journalData: AppJournalDay | null = null

  @state()
  error = ""

  async loadCachedData(): Promise<string | null> {
    const storage = await Storage.init(this.keys!.public_key_hash())
    // TODO lastId should be equal to the last id received from `find` call, not last saved entry
    //      Imagine we made `find` at time=1, then after some time we added an entry at time=3.
    //      Next `find` call should use time=1 as an timestamp, not time=3
    let lastId = null
    let loaded = 0
    for (const entry of await storage.values()) {
      lastId = entry.key
      this.app!.add_entry(entry.value)
      loaded++
    }
    info(`Loaded ${loaded} entries from cache with last one ${lastId}`)
    return lastId
  }

  async loadServerData(lastId: string | null) {
    const storage = await Storage.init(this.keys!.public_key_hash())
    const start = performance.now()
    // TODO Probably should be also outside of the component
    const lines = await find(this.keys!, lastId)
    const requestFinished = performance.now()
    const decrypted = await this.encryptionPool!.decryptAll(lines)
    const end = performance.now()
    info(
      `${decrypted.length} entries loaded in ${Math.floor(end - start)}ms. API=${Math.floor(
        requestFinished - start
      )}ms Decryption=${Math.floor(end - requestFinished)}ms`
    )
    for (const entry of decrypted) {
      this.app!.add_entry(entry.text)
      await storage.setItem(entry.id, entry.text)
    }
  }

  async connectedCallback() {
    super.connectedCallback()
    try {
      const lastId = await this.loadCachedData()
      await this.loadServerData(lastId)
      this.journalData = this.app!.journal_day(DateDay.fromDate(this.today))
    } catch (ex) {
      this.error = String(ex)
      throw ex
    }
  }

  switchDay(diff: number) {
    const newDay =
      diff > 0 ? this.journalData!.day.add_days(1) : this.journalData!.day.remove_days(1)
    this.journalData = this.app!.journal_day(newDay)
  }

  render() {
    return html`
      <q-logo-block>
        <h1>Progress</h1>
        <q-journal
          .data=${this.journalData}
          .keys=${this.keys}
          @next=${() => this.switchDay(1)}
          @prev=${() => this.switchDay(-1)}
        ></q-journal>
        <q-skills .data=${this.app?.view_skills().skills}></q-skills>
        ${this.error && html`<p>Error ${this.error}</p>`}
      </q-logo-block>
    `
  }
}
