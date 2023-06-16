import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"
import "../controls/panel"

declare global {
  interface HTMLElementTagNameMap {
    "q-skills": Skills
  }
}

@customElement("q-skills")
export class Skills extends LitElement {
  @property({ type: String })
  data!: string

  static styles = css`
    .journal h2 {
      text-align: center;
    }
    .journal {
      text-align: left;
    }
  `

  render() {
    const skills = this.data.split("\n")
    return html`<q-panel title="Skills">
      <div class="skills">${skills.map((v) => html`<p>${v}</p>`)}</div></q-panel
    >`
  }
}
