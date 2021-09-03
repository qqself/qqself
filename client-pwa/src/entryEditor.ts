import {css, html, LitElement} from 'lit';
import {customElement, property} from 'lit/decorators.js';

@customElement("q-entry-editor")
class EntryEditor extends LitElement {
    static styles = css`
    .uncategorized { border: solid 1px black }
    `

    @property()
    text = ""

    @property()
    start = ""

    @property()
    end = ""

    render() {
        return html`
            <div class="editor">
                <q-entry-input text="${this.text}"></q-entry-input>
                <div>
                    Start <input type="text" value="${this.start}"/>
                </div>
                <div>
                    End <input type="text" value="${this.end}"/>
                </div>
                <button>Save</button>
            </div>`
    }
}
