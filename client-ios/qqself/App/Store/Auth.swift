import qqselfCoreLib

let storageKeysKey = "keys"  // Global key to store current user cached auth keys

struct Auth {

  static func onInitStarted(_ store: Store) async {
    Log.info(buildInfo())
    setPanicHook()

    do {
      let pool = try await getCredentials(persistent: config.isDataPersistent)
      await store.dispatch(EventType.InitSucceeded(cryptor: pool))
    } catch {
      Log.error("Error in initStarted: " + error.localizedDescription)
      await store.dispatch(EventType.InitErrored(error: error))
    }
  }

  static func onInitSucceeded(_ store: Store, pool: CryptorPool?) async {
    if let pool = pool {
      // Automatically login user if there are cached keys and cryptor from the previous session
      await store.dispatch(EventType.AuthLoginSucceeded(cryptor: pool))
    } else {
      await store.dispatch(EventType.AuthLoginNotAuthenticated())
    }
  }

}

private func getCredentials(persistent: Bool) async throws -> CryptorPool? {
  let storage = try newDefaultStorage(persistent: persistent)
  let cachedKeys = try await storage.getItem(storageKeysKey)
  if let cachedKeys {
    return try CryptorPool.fromKeys(cachedKeys)
  }
  return nil
}

private func saveCredentials(persistent: Bool, pool: CryptorPool) async throws {
  let storage = try newDefaultStorage(persistent: persistent)
  return try await storage.setItem(key: storageKeysKey, value: pool.serialiseKeys())
}

private func deleteCredentials(persistent: Bool) async throws {
  let storage = try newDefaultStorage(persistent: persistent)
  return try await storage.removeItem(storageKeysKey)
}
