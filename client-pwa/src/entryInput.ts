import {html, css, LitElement} from 'lit';
import {customElement, property} from 'lit/decorators.js';
import {parse} from "@rsw/wrapper";

@customElement('entry-input')
export class EntryInput extends LitElement {
    static styles = css`p { color: blue }`;

    @property()
    text = 'Somebody';

    render() {
        console.log("Parsing")
        const parsed = parse("tag1 prop1 val2")
        console.log("PARSED " + parsed)
        const text = "${this.text}! Parsed ${parsed}"
        return html`Nope <input type="text" value=${parsed}/>`;
    }
}
