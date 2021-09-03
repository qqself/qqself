import {css, html, LitElement} from 'lit';
import {customElement, property} from 'lit/decorators.js';

@customElement("q-day-summary")
class DaySummary extends LitElement {
    static styles = css`
    .uncategorized { border: solid 1px black }
    `

    @property()
    uncategorized = ""

    render() {
        return html`
            <div class="summary">
                <h2>Day summary</h2>
                <div class="uncategorized">Uncategorized ${this.uncategorized}</div>
                <slot></slot>
                <button>Add</button>
                <button>Search</button>
            </div>`
    }
}
