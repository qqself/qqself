import CoreData

protocol Storage {
  func clear() async throws
  func itemCount() async throws -> Int
  func getItem(_ key: String) async throws -> String?
  func setItem(key: String, value: String) async throws
  func removeItem(_ key: String) async throws
  func values(keyPrefix: String) async throws -> [String: String]
}

func newStorage(dbName: String, persistent: Bool) throws -> Storage {
  if persistent {
    return try SQLiteDatabase(dbName: dbName)
  } else {
    return MemoryStorage(dbName: dbName)
  }
}

// Returns "DEFAULT" database. Used as a global persistent storage for
// storing information about other accounts and databases
func newDefaultStorage(persistent: Bool) throws -> Storage {
  return try newStorage(dbName: "DEFAULT", persistent: persistent)
}
