import { html, LitElement } from "lit"
import { customElement, state } from "lit/decorators.js"
import init, { initialize, Keys } from "../../bridge/pkg"
import { log } from "../logger"
import "../components/logoBlock"
import { EncryptionPool } from "../encryptionPool"
import { Storage } from "../storage"

declare global {
  interface HTMLElementTagNameMap {
    "q-loading-page": LoadingPage
  }
}

export type LoadedEvent = CustomEvent<{ encryptionPool: EncryptionPool; keys: Keys | null }>

@customElement("q-loading-page")
export class LoadingPage extends LitElement {
  @state()
  loaded = false

  @state()
  errors = ""

  async firstUpdated() {
    log("Checking requirements")
    const missingApi = this.checkMissingAPI()
    if (missingApi) {
      this.errors = missingApi
      return
    }
    try {
      await init()
      const storage = await Storage.initDefault(true)
      const cachedKeys = await storage.getItem("keys")
      const keys = cachedKeys ? Keys.deserialize(cachedKeys) : null
      setTimeout(() => {
        this.loaded = true
        const config = initialize()
        log(`Initialized:\n${config}`)
        const encryptionPool = new EncryptionPool()
        const event: LoadedEvent = new CustomEvent("loaded", {
          detail: { encryptionPool, keys },
        })
        this.dispatchEvent(event)
      }, 500) // TODO There has to be a better way to solve flickering of UI when checks completes very fast
    } catch (ex: any) {
      this.errors = ex.toString()
      return
    }
  }

  checkMissingAPI() {
    if (!window.Worker) {
      return "Worker browser API is not supported. We need it for running encryption/decryption without blocking UI. Please update your browser"
    }
    if (!window.WebAssembly) {
      return "WebAssembly API is not supported. We need it as core of our app requires it. Please update your browser"
    }
    // WebAssembly loading may still fail, do some extensive checks. Based on https://stackoverflow.com/a/47880734
    try {
      const module = new WebAssembly.Module(
        Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00)
      )
      new WebAssembly.Instance(module)
    } catch (e: any) {
      return "WebAssembly API is not supported:" + e.toString()
    }
    return null
  }

  statusElement() {
    if (this.errors) {
      return html`<h1>Error</h1>
        <p>${this.errors}</p>`
    }
    if (!this.loaded) {
      return html`<h1>Loading...</h1>`
    }
    return html`<h1>Ready</h1>`
  }

  render() {
    return html`<q-logo-block>${this.statusElement()}</q-logo-block>`
  }
}
