import CoreData

protocol Storage {
    func clear() async throws
    func itemCount() async throws -> Int
    func getItem(_ key: String) async throws -> String?
    func setItem(key: String, value: String) async throws 
    func removeItem(_ key: String) async throws
    func values(keyPrefix: String) async throws -> [String: String]
}

func newStorage(dbName: String, persistant: Bool) throws ->Storage {
    if persistant {
        return try SQLiteDatabase(dbName: dbName)
    } else {
        return MemoryStorage(dbName: dbName)
    }
}
