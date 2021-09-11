import "./entryInput"
import "./devcards"
import "./entryList"
import {css, html, LitElement} from "lit"
import {customElement, state} from "lit/decorators.js"
import init from "@rsw/wrapper"
import {log} from "./logger"
import {MemoryStorage} from "./storage"

type Page = "main" | "devcards"
type AddMode = "activity" | "data" | "reflexion"

@customElement("q-main")
export class Main extends LitElement {
  static styles = css`
    .container {
      display: flex;
      justify-content: space-between;
    }
    .active {
      border: 1px solid Black;
    }
    h3 {
      width: 100%;
      text-align: center;
    }
    a:visited {
      color: blue;
    }
  `

  @state()
  initialized = false

  @state()
  addMode: AddMode = "activity"

  @state()
  entries = ""

  constructor() {
    super()
    ;(async () => {
      log("Initializing...")
      if ("serviceWorker" in navigator) {
        await navigator.serviceWorker.register("sw.js")
      }
      await init()
      const entries = await new MemoryStorage().read()
      this.initialized = true
      this.entries = entries
      log(`Initialization done, entries ${this.entries}`)
    })()
  }

  route(): Page {
    const hash = window.location.hash.slice(1)
    if (hash == "") {
      return "main"
    }
    return "devcards"
  }

  renderDevcards() {
    return html`<q-devcards></q-devcards>`
  }

  onClick = (sender: AddMode) => {
    return () => {
      this.addMode = sender
    }
  }

  onEntryAdded = async (text: string) => {
    console.log("Add to storage: ", text)
    const entries = this.entries + "\n" + text
    await new MemoryStorage().write(entries)
    this.entries = entries
    console.log(`Local storage updated ${this.entries}`)
  }

  renderAddElement() {
    switch (this.addMode) {
      case "activity":
        return html`
          <h3>Add ${this.addMode}</h3>
          <q-entry-list mode="recent" text=${this.entries}></q-entry-list>
          <q-entry-input mode="string" .onText=${this.onEntryAdded}></q-entry-input>
        `
      case "data":
        return html`
          <h3>Add ${this.addMode}</h3>
          <q-entry-input mode="text" .onText=${this.onEntryAdded}></q-entry-input>
        `
      case "reflexion":
        return html`
          <h3>Add ${this.addMode}</h3>
          <div style="display: flex; justify-content: space-between">
            <a href="">Day</a>
            <a href="">Week</a>
            <a href="">Month</a>
          </div>
          <br />
          <q-entry-input mode="text" .onText=${this.onEntryAdded}></q-entry-input>
        `
    }
  }

  renderMain() {
    const btnClasses = {[this.addMode]: "active"}
    return html`
      <div class="container">
        <a href="">Search</a>
        <a href="">Dashboard</a>
        <a href="">Shortcuts</a>
      </div>
      <hr />
      <div class="container">
        <button @click=${this.onClick("activity")} class="${btnClasses["activity"]}">
          Add Activity
        </button>
        <button @click=${this.onClick("data")} class="${btnClasses["data"]}">Add Data</button>
        <button @click=${this.onClick("reflexion")} class="${btnClasses["reflexion"]}">
          Add Reflexion
        </button>
      </div>
      ${this.renderAddElement()}
    `
  }

  render() {
    if (!this.initialized) {
      return html`Loading...`
    }
    const page = this.route()
    log(`Rendering ${page}`)
    switch (page) {
      case "main":
        return this.renderMain()
      case "devcards":
        return this.renderDevcards()
    }
  }
}
