import "../controls/panel"
import "../controls/icon"

import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"

import { SkillData } from "../../../qqself_core"
import { IconName } from "../controls/icon"

declare global {
  interface HTMLElementTagNameMap {
    "q-skills": Skills
  }
}

@customElement("q-skills")
export class Skills extends LitElement {
  @property({ type: Array })
  skills: SkillData[] = []

  static styles = css`
    q-icon {
      --icon-size: 15px;
    }
    .skill {
      display: flex;
      justify-content: space-evenly;
    }
    .icon {
      display: flex;
      flex-basis: 5%;
    }
    .title {
      display: flex;
      flex-basis: 80%;
    }
    .level {
      display: flex;
      flex-basis: 5%;
      justify-content: right;
    }
  `

  skillIcon(kind: string) {
    let icon: IconName | null = null
    if (kind == "physical") {
      icon = "activityPhysical"
    } else if (kind == "intelligent") {
      icon = "activityIntelligent"
    } else if (kind == "creative") {
      icon = "activityCreative"
    }
    if (icon) {
      return html`<q-icon class="icon" name=${icon}></q-icon>`
    }
  }

  render() {
    return html`<q-panel title="Identities">
      <div class="skills">
        ${this.skills.map(
          (skill) =>
            html` <div class="skill">
              ${this.skillIcon(skill.kind)}
              <span class="title">${skill.title}</span>
              <span class="level">${skill.level}</span>
            </div>`,
        )}
      </div></q-panel
    >`
  }
}
