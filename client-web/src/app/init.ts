import init, { initialize, Keys } from "../../qqself_core"
import { info } from "../logger"
import { isBrowser } from "../utils"
import { getCredentials } from "./auth"
import { Store } from "./store"

export const started = async (store: Store): Promise<void> => {
  const apiCheckError = checkMissingAPI()
  if (apiCheckError) {
    return store.dispatch("init.errored", { error: new Error(apiCheckError) })
  }
  // In browser context we need to initiate WebAssembly
  if (isBrowser) {
    await init()
  }
  info(initialize())
  const keys = await getCredentials()
  return store.dispatch("init.succeeded", { cachedKeys: keys })
}

export const succeeded = (store: Store, cachedKeys: Keys | null): Promise<void> => {
  // There are cached keys, automatically login
  if (cachedKeys) {
    return store.dispatch("auth.login.succeeded", { keys: cachedKeys })
  } else {
    return store.dispatch("auth.login.notAuthenticated", null)
  }
}

const checkMissingAPI = () => {
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
