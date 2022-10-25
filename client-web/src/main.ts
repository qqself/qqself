import { html, LitElement } from "lit";
import { customElement, state } from "lit/decorators.js";
import { log } from "./logger";
import init, { foo } from "../core/pkg";

@customElement("q-main")
export class Main extends LitElement {
  @state()
  msg = "Loading...";

  constructor() {
    super();
    (async () => {
      log("Initializing...");
      await init();
      this.msg = `Rust calculated hash: ${foo("msg")}`;
      log(`Initialization done`);
    })();
  }

  render() {
    return html` <h1>${this.msg}</h1> `;
  }
}
