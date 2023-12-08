import CoreData

protocol Storage {
    func clear() throws
    func itemCount() throws -> Int
    func getItem(_ key: String) throws -> String?
    func setItem(key: String, value: String) throws
    func removeItem(_ key: String) throws
    func values(keyPrefix: String) throws -> [String: String]
}

func newStorage(dbName: String, persistant: Bool) throws ->Storage {
    if persistant {
        return try SQLiteDatabase(dbName: dbName)
    } else {
        return MemoryStorage(dbName: dbName)
    }
}
