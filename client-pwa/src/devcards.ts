import {css, html, LitElement} from 'lit';
import {customElement, property} from 'lit/decorators.js';
import "./entryInput"
import "./daySymmary"
import "./entryProgress"
import "./entryEditor"
import "./entrySearch"

@customElement("q-card")
class Card extends LitElement {
    static styles = css`
    .card { border: solid 1px black; margin-bottom: 20px; }
    .name { text-align: center }
    .content { margin: 10px; }
    `

    @property()
    name = ""

    render() {
        const filter = window.location.hash.slice(1)
        const filterEnabled = filter != "" && filter != "devcards"
        if (filterEnabled && !this.name.toLowerCase().includes(filter)) {
            return null;
        }
        return html`
            <div class="card">
                <div class="name">${this.name}</div>
                <div class="content">
                    <slot></slot>
                </div>
            </div>`
    }
}

@customElement('q-devcards')
export class Devcards extends LitElement {
    render() {
        return html`
            <div>
                <q-card name="entryInput: Empty">
                    <q-entry-input text=""></q-entry-input>
                </q-card>
                <q-card name="entryInput: With data">
                    <q-entry-input text="tag1 prop1 val1. tag2"></q-entry-input>
                </q-card>
                <q-card name="daySummary: Empty">
                    <q-day-summary uncategorized="1:10m"></q-day-summary>
                </q-card>
                <q-card name="entryProgress: Zero">
                    <q-entry-progress progress="0" name="Work"></q-entry-progress>
                </q-card>
                <q-card name="entryProgress: Partial">
                    <q-entry-progress progress="85" name="Work"></q-entry-progress>
                </q-card>
                <q-card name="entryProgress: Complete">
                    <q-entry-progress progress="100" name="Work"></q-entry-progress>
                </q-card>
                <q-card name="entryEditor: Simple">
                    <q-entry-editor text="tag1 prop1 val2" start="11:20" end="11:56"></q-entry-editor>
                </q-card>
                <q-card name="entrySearch: Found">
                    <q-entry-search query="tag1" .data=${[
                        {name: "January", time: 22},
                        {name: "February", time: 49},
                        {name: "March", time: 12},
                        {name: "April", time: 24}
                    ]}></q-entry-search>
                </q-card>
            </div>`;
    }
}
