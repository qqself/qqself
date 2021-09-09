import {css, html, LitElement} from "lit"
import {customElement, property, query, state} from "lit/decorators.js"
import {parse} from "@rsw/wrapper"

@customElement("q-entry-input")
export class EntryInput extends LitElement {
  static styles = css`
    .container {
      display: flex;
      justify-content: space-between;
    }
    .input {
      width: 80%;
    }
    .text {
      margin: auto;
      display: block;
      width: 95%;
      min-height: 400px;
    }
    .textBtn {
      float: right;
      width: 100px;
      margin-top: 20px;
    }
  `

  @query("input", true)
  input!: HTMLInputElement

  @query("textarea", true)
  textarea!: HTMLInputElement

  @property()
  onText: (text: string) => void = () => {}

  @property()
  mode: "string" | "text" = "string"

  onEnterInput() {
    const entered = this.input.value.trim()
    const parsed = parse(entered)
    this.onText(parsed)
    this.input.value = ""
  }

  onEnterText() {
    const entered = this.textarea.value.trim()
    const parsed = parse(entered)
    this.onText(parsed)
    this.textarea.value = ""
  }

  render() {
    if (this.mode == "string") {
      return html` <div class="container">
        <input class="input" type="text" />
        <button @click=${this.onEnterInput}>Add</button>
      </div>`
    }
    return html`
      <div>
        <textarea class="text"></textarea>
        <button class="text_btn" @click=${this.onEnterText}>Add</button>
      </div>
    `
  }
}
