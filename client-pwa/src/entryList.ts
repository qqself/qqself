import "./entryInput"
import {css, html, LitElement} from "lit"
import {customElement, property} from "lit/decorators.js"

@customElement("q-entry-list")
class EntryList extends LitElement {
  static styles = css``

  @property()
  mode: "recent" = "recent"

  @property()
  text = ""

  render() {
    const lines = this.text.split("\n").slice(this.mode == "recent" ? -20 : 0)
    return html` <div class="list">
      <ul>
        ${lines.map(v => html` <li>${v}</li>`)}
      </ul>
    </div>`
  }
}
