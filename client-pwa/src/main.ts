import "./entryInput";
import "./devcards"
import {css, html, LitElement} from 'lit';
import {customElement, state} from 'lit/decorators.js';
import init from "@rsw/wrapper";
import {log} from "./logger"

type Page = 'main' | 'devcards'

@customElement('q-main')
export class Main extends LitElement {
    static styles = css`p { color: blue }`;

    @state()
    initialized = false;

    constructor() {
        super();
        (async () => {
            // Init Rust parsers and anything else on first load
            log("Initializing...")
            await init()
            log("Initialization done")
            this.initialized = true;
        })()
    }

    route(): Page {
        const hash = window.location.hash.slice(1)
        if (hash == "") {
            return "main"
        }
        return "devcards"
    }

    renderDevcards() {
        return html`<q-devcards></q-devcards>`
    }

    renderMain() {
        return html`
            <p>
                Main page with input
                <q-entry-input text="Hello World"></q-entry-input>
            </p>
        `;
    }

    render() {
        // WebAssembly not loaded yet, wait
        if (!this.initialized) {
            return html`Loading...`
        }

        const page = this.route()
        log(`Rendering ${page}`)
        switch (page) {
            case "main":
                return this.renderMain()
            case "devcards":
                return this.renderDevcards()
        }

    }
}
