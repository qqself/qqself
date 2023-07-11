import { Views, Keys } from "../../bridge/pkg/qqself_client_web_bridge"
import { EncryptionPool } from "./encryptionPool/pool"
import { Store, ViewNotification, ViewUpdate } from "./store"
import * as Storage from "./storage/storage"

export const loginSucceeded = async (store: Store, keys: Keys): Promise<void> => {
  await saveCredentials(keys)
  const onViewUpdate = (data: Map<string, string>) => {
    const update = Object.fromEntries(data) as unknown as ViewUpdate
    if (update.view == "Journal") {
      void store.dispatch("views.update.journal", { update })
    } else {
      void store.dispatch("views.update.skills", { update })
    }
  }
  const onViewNotification = (data: Map<string, string>) => {
    const update = Object.fromEntries(data) as unknown as ViewNotification
    void store.dispatch("views.notification.skills", { update })
  }
  store.userState = {
    encryptionPool: EncryptionPool.initWithKeys(keys),
    storage: Storage.newStorage(keys.public_key_hash()),
    // HACK This callback is called from `Views.add_entry` which captured Views as
    //      `&mut self`. If any store subscribers for the following events will try
    //      to call Views functions with capturing &self then Rust/WS will break.
    //      setTimeout allows callback to complete, freeing `Views &mut self` and
    //      schedules actual callback logic for the next event loop cycle
    views: Views.new(
      keys,
      (data: Map<string, string>) => setTimeout(() => onViewUpdate(data), 0),
      (data: Map<string, string>) => setTimeout(() => onViewNotification(data), 0),
    ),
  }
}

export const login = (store: Store, keyString: string): Promise<void> => {
  try {
    const keys = Keys.deserialize(keyString)
    return store.dispatch("auth.login.succeeded", { keys })
  } catch (e) {
    return store.dispatch("auth.login.errored", { error: new Error(String(e)) })
  }
}

export const newKeys = async (): Promise<Keys> => {
  const pool = EncryptionPool.initKeyless()
  return pool.generateNewKeys()
}

export const registrationStarted = async (
  store: Store,
  mode: "interactive" | "automatic",
): Promise<void> => {
  if (mode == "automatic") {
    const keys = await newKeys()
    return store.dispatch("auth.registration.succeeded", { keys })
  } else {
    // In interactive mode user has to create a keys and download it
    // which happens via interactive experience, do nothing here
  }
}

export const registrationSucceeded = async (store: Store, keys: Keys): Promise<void> => {
  return store.dispatch("auth.login.succeeded", { keys })
}

export const logoutStarted = async (store: Store): Promise<void> => {
  await deleteCredentials()
  return store.dispatch("auth.logout.succeeded", null)
}

export const logoutSucceeded = async (store: Store): Promise<void> => {
  store.userState = {} as never // Reset whole user state and start over
  return Promise.resolve()
}

const STORAGE_KEYS_KEY = "keys"
export const getCredentials = async (): Promise<Keys | null> => {
  const storage = Storage.newDefaultStorage()
  const cachedKeys = await storage.getItem(STORAGE_KEYS_KEY)
  return cachedKeys ? Keys.deserialize(cachedKeys) : null
}

export const saveCredentials = async (keys: Keys): Promise<void> => {
  const storage = Storage.newDefaultStorage()
  return storage.setItem(STORAGE_KEYS_KEY, keys.serialize())
}

export const deleteCredentials = async (): Promise<void> => {
  const storage = Storage.newDefaultStorage()
  return storage.removeItem(STORAGE_KEYS_KEY)
}
