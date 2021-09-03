import {css, html, LitElement} from 'lit';
import {customElement, property} from 'lit/decorators.js';
import '@google-web-components/google-chart';

interface Item {
    time: number
    name: string
}

@customElement("q-entry-search")
class EntrySearch extends LitElement {
    static styles = css`
    .filter { margin: 10px 0; }
    `
    @property()
    query = ""

    @property()
    data = [] as Item[]

    time() {
        const total = this.data.reduce((acc, cur) => acc + cur.time, 0)
        if (total < 60) {
            return `${total} min`
        }
        const hours = Math.floor(total / 60);
        const minutes = total - (hours * 60)
        return `${hours}:${minutes}`
    }

    render() {
        const vals = this.data.map(v => `["${v.name}",${v.time}]`).join(',')
        return html`
            <div class="search">
                <h2>Search</h2>
                <q-entry-input text="${this.query}"></q-entry-input>
                <div class="filter">
                    <input type="radio" name="filter" id="all" value="all" checked>
                    <label for="all">All</label>
                    
                    <input type="radio" name="filter" id="year" value="year">
                    <label for="year">Last year</label>

                    <input type="radio" name="filter" id="month" value="month">
                    <label for="month">Last month</label>

                    <input type="radio" name="filter" id="week" value="week">
                    <label for="week">Last week</label>
                </div>
                <div class="summary">
                    Count: ${this.data.length}
                    <br>
                    Time: ${this.time()}
                </div>
                <div class="chart">
                    <google-chart data='[["Period","Time"],${vals}]'></google-chart>
                </div>
            </div>`
    }
}
