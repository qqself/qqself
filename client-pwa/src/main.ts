import "./entryInput";
import {css, html, LitElement} from 'lit';
import {customElement, property, state} from 'lit/decorators.js';
import init from "@rsw/wrapper";

@customElement('qqself-hello')
export class QQSelfHello extends LitElement {
    static styles = css`p { color: blue }`;

    constructor() {
        super();
        (async () => {
            console.log("Before loading...")
            await init()
            console.log("Loaded")
            this.loaded = true;
        })()
    }

    @property()
    name = 'Somebody';

    @state()
    loaded = false;

    render() {
        if (!this.loaded) {
            return html`Loading...`
        }
        return html`
            <p>
                Main24 tsaaa? ${this.name}!
                <entry-input text=${this.name}></entry-input>
            </p>
        `;
    }
}
