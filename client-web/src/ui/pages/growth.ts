import "../components/logoBlock"
import "../controls/button"
import "../controls/notification"
import "../components/queryResults"
import "../components/skills"
import "../components/week"
import "../components/statusBar"

import { css, html, LitElement } from "lit"
import { customElement, property, state } from "lit/decorators.js"

import { Store } from "../../app/store"
import { PostNoteSize } from "../components/postNote"

declare global {
  interface HTMLElementTagNameMap {
    "q-growth-page": GrowthPage
  }
}

interface Rejection {
  day: Date
  text: string
}

interface Point {
  left: number
  top: number
}

@customElement("q-growth-page")
export class GrowthPage extends LitElement {
  @property({ type: Object })
  store!: Store

  @state()
  rejections: Rejection[] = []

  @state()
  positions: Point[] = []

  static styles = css`
    .root {
      padding: 10px;
      height: 100%;
      position: relative;
    }
    .rejections {
      height: 100%;
    }
    .rejection {
      position: absolute;
    }
  `

  // We want an even spread of random placements across the whole area
  // We split available area to the grid and for new placement search
  // for least populated grid cells and place an item there adding
  // a random offset for better look
  updateWidthAndHeight() {
    const { width, height } = this.getBoundingClientRect()
    const gridColumns = Math.max(Math.floor(width / PostNoteSize), 2)
    const gridRows = Math.max(Math.round(height / PostNoteSize), 2)
    const grid = Array.from({ length: gridColumns }, () => Array(gridRows).fill(0) as number[])
    const positions: Point[] = []
    const jitter = () => {
      const max = 15
      const min = -15
      return Math.random() * (max - min) + min
    }
    this.rejections.forEach(() => {
      // Find next least populated cell for the next placement
      let min = Infinity
      let cells: { column: number; row: number }[] = []
      for (let i = 0; i < grid.length; i++) {
        for (let y = 0; y < grid[i].length; y++) {
          if (grid[i][y] < min) {
            min = grid[i][y]
            cells = [{ column: i, row: y }]
          } else if (grid[i][y] == min) {
            cells.push({ column: i, row: y })
          }
        }
      }
      // Calculate positions based on a found cell
      const cell = cells[Math.floor(Math.random() * cells.length)]
      const margin = 30 // To ensure to avoid hitting edges
      const top = cell.row * PostNoteSize + margin + jitter()
      let left = cell.column * PostNoteSize + margin + jitter()
      // To avoid horizontal scrolling move placement back if we exceed or too close to the right border
      const rightMargin = 50
      if (left + PostNoteSize + rightMargin > width) {
        left -= left + PostNoteSize + rightMargin - width
      }
      positions.push({ left, top })
      grid[cell.column][cell.row]++
    })
    this.positions = positions
  }

  firstUpdated() {
    this.updateWidthAndHeight()
    window.addEventListener("resize", this.updateWidthAndHeight.bind(this))
  }

  connectedCallback() {
    super.connectedCallback()
    this.rejections = [
      {
        day: new Date("2022-02-02"),
        text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam",
      },
      {
        day: new Date("2022-02-03"),
        text: "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur",
      },
      {
        day: new Date("2022-02-03"),
        text: "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum",
      },
      {
        day: new Date("2022-02-05"),
        text: "Sed ut perspiciatis, unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam eaque ipsa, quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt, explicabo",
      },
      { day: new Date("2022-02-06"), text: "Nemo enim ipsam voluptatem, quia voluptas sit" },
      {
        day: new Date("2022-02-11"),
        text: "At vero eos et accusamus et iusto odio dignissimos ducimus",
      },
      {
        day: new Date("2022-04-12"),
        text: "Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet, ut et voluptates repudiandae sint et molestiae non recusandae",
      },
      {
        day: new Date("2022-02-02"),
        text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam",
      },
      {
        day: new Date("2022-02-03"),
        text: "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur",
      },
      {
        day: new Date("2022-02-03"),
        text: "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum",
      },
      {
        day: new Date("2022-02-05"),
        text: "Sed ut perspiciatis, unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam eaque ipsa, quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt, explicabo",
      },
      { day: new Date("2022-02-06"), text: "Nemo enim ipsam voluptatem, quia voluptas sit" },
      {
        day: new Date("2022-02-11"),
        text: "At vero eos et accusamus et iusto odio dignissimos ducimus",
      },
      {
        day: new Date("2022-04-12"),
        text: "Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet, ut et voluptates repudiandae sint et molestiae non recusandae",
      },
    ]
  }

  renderRejection(rejection: Rejection, index: number) {
    const position = this.positions[index] || { left: 0, top: 0 }
    return html`<div class="rejection" style=${`left: ${position.left}px; top: ${position.top}px;`}>
      <q-post-note .text=${rejection.text}></q-post-note>
    </div>`
  }

  render() {
    return html` <div class="root">
      <div class="rejections">${this.rejections.map((v, idx) => this.renderRejection(v, idx))}</div>
    </div>`
  }
}
