import {html, LitElement} from 'lit';
import {customElement, property} from 'lit/decorators.js';

@customElement("q-entry-progress")
class EntryProgress extends LitElement {
    @property()
    name = ""

    @property()
    progress = 0

    render() {
        return html`
            <div class="progress">
                <div class="name">${this.name}</div>
                <progress value="${this.progress}" max="100"></progress>
            </div>`
    }
}
