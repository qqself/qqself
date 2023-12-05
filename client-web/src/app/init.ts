import init, { Cryptor, initialize } from "../../qqself_core"
import { info } from "../logger"
import { isBrowser } from "../utils"
import { getCredentials } from "./auth"
import { Store } from "./store"

export const started = async (store: Store): Promise<void> => {
  const apiCheckError = checkBrowserAPI()
  if (apiCheckError) {
    return store.dispatch("init.errored", { error: new Error(apiCheckError) })
  }
  // In browser context we need to initiate WebAssembly
  if (isBrowser) {
    await init()
  }
  info(initialize())
  const cryptor = await getCredentials()
  return store.dispatch("init.succeeded", { cryptor })
}

export const succeeded = (store: Store, cryptor: Cryptor | null): Promise<void> => {
  // There is cached Cryptor from previous session, automatically login
  if (cryptor) {
    return store.dispatch("auth.login.succeeded", { cryptor })
  } else {
    return store.dispatch("auth.login.notAuthenticated", null)
  }
}

const checkBrowserAPI = () => {
  // WebAssembly loading may fail, do some extensive checks to ensure it works
  // Based on https://stackoverflow.com/a/47880734
  try {
    const module = new WebAssembly.Module(
      Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00),
    )
    new WebAssembly.Instance(module)
  } catch (e) {
    return `WebAssembly API is not supported: ${String(e)}`
  }
  return null
}
