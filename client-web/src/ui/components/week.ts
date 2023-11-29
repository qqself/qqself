import "../controls/panel"
import "../controls/icon"

import { css, html, LitElement } from "lit"
import { customElement, property } from "lit/decorators.js"

import { SkillWeek } from "../../../qqself_core"

declare global {
  interface HTMLElementTagNameMap {
    "q-week-view": WeekView
  }
}

@customElement("q-week-view")
export class WeekView extends LitElement {
  @property({ type: Array })
  data: SkillWeek[] = []

  static styles = css`
    q-icon {
      --icon-size: 15px;
      margin-top: 2px;
    }
    .skill {
      display: flex;
      justify-content: space-between;
    }
    .skill .name {
      display: flex;
      justify-content: flex-start;
    }
    .titleProgress {
      display: flex;
      flex-direction: column;
    }
    .icon {
      width: 20px;
    }
    .progressBar {
      width: 100%;
      margin-left: 20px;
      display: block;
      margin-top: -8px;
    }
    .started {
      margin-left: 20px;
    }
    .noProgressBar {
      height: 10px;
    }
  `

  renderSkill(skill: SkillWeek, index: number) {
    const icon = html`<q-icon
      class="icon"
      name="${skill.progress > 0 ? "check" : "unchecked"}"
    ></q-icon>`
    const percent = progressPercent(skill.progress, skill.target)
    let progress = ""
    if (percent >= 100) {
      progress = "Perfect"
    } else if (percent > 0) {
      progress = `${percent}% of perfection`
    }
    const progressBar =
      percent > 0 && percent < 100
        ? html`<progress class="progressBar" value=${percent} max="100"></progress>`
        : index != this.data.length - 1
          ? html`<div class="noProgressBar" />`
          : null
    return html`<div
      class="skillRow"
      title=${`${toHours(skill.progress)} from ${toHours(skill.target)}`}
    >
      <div class="skill">
        <div class="name">
          ${icon}
          <div class="titleProgress">
            <span class="title">${skill.name}</span>
          </div>
        </div>
        <span class="progress">${progress}</span>
      </div>
      ${progressBar}
    </div>`
  }

  renderStart() {
    const total = this.data.length
    const started = this.data.filter((v) => v.progress > 0).length
    const msg = total == started ? "Perfect start" : `Started ${started} of ${total}`
    return html`<span class="started">${msg}</span>`
  }

  render() {
    return html`<q-panel title="Week">
      <div class="root">
        ${this.data.sort(sortByProgress).map((v, i) => this.renderSkill(v, i))}
        <hr />
        ${this.renderStart()}
      </div></q-panel
    >`
  }
}

const progressPercent = (current: number, target: number): number => {
  return Math.round(Math.round((current / target) * 100) / 5) * 5
}

const sortByProgress = (a: SkillWeek, b: SkillWeek): number => {
  return progressPercent(a.progress, a.target) - progressPercent(b.progress, b.target)
}

const toHours = (minutes: number): string => {
  const hours = Math.floor(minutes / 60)
  const left = String(minutes - hours * 60).padStart(2, "0")
  return `${hours}:${left}`
}
