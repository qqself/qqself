import {html, LitElement} from 'lit';
import {customElement, property} from 'lit/decorators.js';
import {parse} from "@rsw/wrapper";

@customElement('q-entry-input')
export class EntryInput extends LitElement {
    @property()
    text = '';

    render() {
        const parsed = parse(this.text)
        return html`EntryInput <input type="text" value=${parsed}/>`;
    }
}
