// Global mutable variable to store the data while process is running
// to behave closer to persistent storage and make testing easier
var data: [String: [String: String]] = [:]

// In memory data storage, for testing and deveopment
struct MemoryStorage : Storage {
    let dbName: String

    init(dbName: String) {
        self.dbName = dbName
        if (data[dbName] == nil) {
            data[dbName] = [:]
        }
    }
    
    func clear() {
        data[dbName] = [:]
    }
    
    func itemCount() -> Int {
        return data[dbName]?.count ?? 0
    }
    
    func getItem(_ key: String) -> String? {
        return data[dbName]?[key]
    }
    
    func setItem(key: String, value: String) {
        data[dbName]?[key] = value
    }
    
    func removeItem(_ key: String) {
        data[dbName]?[key] = nil
    }
    
    func values(keyPrefix: String) -> [String: String] {
        let items = (data[dbName] ?? [:])
            .filter { $0.key.hasPrefix(keyPrefix) }
            .sorted { $0.key < $1.key }
        var output: [String: String] = [:]
        for (key, value) in items {
            output[key] = value
        }
        return output
    }
    
}
