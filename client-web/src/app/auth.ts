import { Cryptor, Views } from "../../qqself_core"
import { CryptorPool } from "./cryptorPool/pool"
import { DataEvents } from "./data"
import * as Storage from "./storage/storage"
import { Store, ViewNotification, ViewUpdate } from "./store"

export const loginSucceeded = async (store: Store, cryptor: Cryptor): Promise<void> => {
  await saveCredentials(cryptor)

  const onViewUpdate = (data: Map<string, string>) => {
    const update = Object.fromEntries(data) as unknown as ViewUpdate
    if (update.view == "QueryResults") {
      void store.dispatch("views.update.queryResults", { update })
    } else if (update.view == "Skills") {
      void store.dispatch("views.update.skills", { update })
    } else {
      void store.dispatch("views.update.week", { update })
    }
  }

  const onViewNotification = (data: Map<string, string>) => {
    const update = Object.fromEntries(data) as unknown as ViewNotification
    void store.dispatch("views.notification.skills", { update })
  }

  // WASM and Cryptor are ready and we can cache user state
  store.userState = {
    encryptionPool: CryptorPool.initWithCryptor(cryptor),
    storage: Storage.newStorage(cryptor.public_key_hash()),
    // HACK This callback is called from `Views.add_entry` which captured Views as
    //      `&mut self`. If any store subscribers for the following events will try
    //      to call Views functions with capturing &self then Rust/WS will break.
    //      setTimeout allows callback to complete, freeing `Views &mut self` and
    //      schedules actual callback logic for the next event loop cycle
    views: Views.new(
      (data: Map<string, string>) => setTimeout(() => onViewUpdate(data), 0),
      (data: Map<string, string>) => setTimeout(() => onViewNotification(data), 0),
    ),
    dataEvents: new DataEvents(store, store.api),
  }
}

export const login = (store: Store, serializedKeys: string): Promise<void> => {
  try {
    const cryptor = Cryptor.from_deserialized_keys(serializedKeys)
    return store.dispatch("auth.login.succeeded", { cryptor })
  } catch (e) {
    return store.dispatch("auth.login.errored", { error: new Error(String(e)) })
  }
}

export const generateCryptor = async (): Promise<Cryptor> => {
  const pool = CryptorPool.initCryptorGenerator()
  return pool.generateCryptor()
}

export const registrationStarted = async (
  store: Store,
  mode: "interactive" | "automatic",
): Promise<void> => {
  if (mode == "automatic") {
    const cryptor = await generateCryptor()
    return store.dispatch("auth.registration.succeeded", { cryptor })
  } else {
    // In interactive mode user has to create a keys and download it
    // which happens via interactive experience, do nothing here
  }
}

export const registrationSucceeded = async (store: Store, cryptor: Cryptor): Promise<void> => {
  return store.dispatch("auth.login.succeeded", { cryptor })
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
export const getCredentials = async (): Promise<Cryptor | null> => {
  const storage = Storage.newDefaultStorage()
  const cachedKeys = await storage.getItem(STORAGE_KEYS_KEY)
  return cachedKeys ? Cryptor.from_deserialized_keys(cachedKeys) : null
}

export const saveCredentials = async (cryptor: Cryptor): Promise<void> => {
  const storage = Storage.newDefaultStorage()
  return storage.setItem(STORAGE_KEYS_KEY, cryptor.serialize_keys())
}

export const deleteCredentials = async (): Promise<void> => {
  const storage = Storage.newDefaultStorage()
  return storage.removeItem(STORAGE_KEYS_KEY)
}
