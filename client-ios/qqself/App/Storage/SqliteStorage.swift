import Foundation
import SQLite3

// Those constants are not exported for some reason from sqlite3.h
let SQLITE_STATIC = unsafeBitCast(0, to: sqlite3_destructor_type.self)
let SQLITE_TRANSIENT = unsafeBitCast(-1, to: sqlite3_destructor_type.self)

enum DatabaseError: Error {
    case openingError(message: String)
    case error(message: String)
}

// SQLite storage wrapper. All IO is happening in background threads to avoid blocking
// main thread and UI. We are using SQLITE_OPEN_FULLMUTEX, so even though IO is happenning
// in background thread execution is serial
class SQLiteDatabase: Storage {
    var db: OpaquePointer?
    
    init(dbName: String) throws {
        let documentDirectory = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!
        let databasePath = documentDirectory.appendingPathComponent("\(dbName).sqlite")
        let flags = SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_FULLMUTEX
        if sqlite3_open_v2(databasePath.path, &db, flags, nil) != SQLITE_OK {
            let errorMessage = String(cString: sqlite3_errmsg(db))
            throw DatabaseError.openingError(message: errorMessage)
        }
        let createTableQuery = """
            CREATE TABLE IF NOT EXISTS Items (
                Key TEXT PRIMARY KEY NOT NULL,
                Value TEXT NOT NULL
            );
        """
        if sqlite3_exec(db, createTableQuery, nil, nil, nil) != SQLITE_OK {
            let errorMessage = String(cString: sqlite3_errmsg(db))
            throw DatabaseError.error(message: errorMessage)
        }
    }

    deinit {
        if sqlite3_close(db) != SQLITE_OK {
            let error = String(cString: sqlite3_errmsg(db))
            fatalError("Error closing database: \(error)")
        }
    }

    func clear() async throws {
        try await Task {
            let clearQuery = "DELETE FROM Items;"
            if sqlite3_exec(db, clearQuery, nil, nil, nil) != SQLITE_OK {
                let errorMessage = String(cString: sqlite3_errmsg(db))
                throw DatabaseError.error(message: errorMessage)
            }
        }.value
    }

    func itemCount() async throws -> Int {
        try await Task {
            let countQuery = "SELECT COUNT(*) FROM Items;"
            var statement: OpaquePointer?
            if sqlite3_prepare_v2(db, countQuery, -1, &statement, nil) == SQLITE_OK {
                defer { sqlite3_finalize(statement) }
                if sqlite3_step(statement) == SQLITE_ROW {
                    let count = Int(sqlite3_column_int(statement, 0))
                    return count
                }
            }
            let errorMessage = String(cString: sqlite3_errmsg(db))
            throw DatabaseError.error(message: errorMessage)
        }.value
    }

    func getItem(_ key: String) async throws -> String? {
        try await Task {
            let getQuery = "SELECT Value FROM Items WHERE Key = ?;"
            var statement: OpaquePointer?
            if sqlite3_prepare_v2(db, getQuery, -1, &statement, nil) == SQLITE_OK {
                defer { sqlite3_finalize(statement) }
                sqlite3_bind_text(statement, 1, key, -1, SQLITE_TRANSIENT)
                let res = sqlite3_step(statement)
                if res == SQLITE_ROW {
                    if let cString = sqlite3_column_text(statement, 0) {
                        let value = String(cString: cString)
                        return value
                    }
                } else if res == SQLITE_DONE {
                    return nil // No key found
                }
            }
            let errorMessage = String(cString: sqlite3_errmsg(db))
            throw DatabaseError.error(message: errorMessage)
        }.value
    }

    func setItem(key: String, value: String) async throws {
        try await Task {
            let setQuery = "INSERT OR REPLACE INTO Items (Key, Value) VALUES (?, ?);"
            var statement: OpaquePointer?
            if sqlite3_prepare_v2(db, setQuery, -1, &statement, nil) == SQLITE_OK {
                defer { sqlite3_finalize(statement) }
                sqlite3_bind_text(statement, 1, key, -1, SQLITE_TRANSIENT)
                sqlite3_bind_text(statement, 2, value, -1, SQLITE_TRANSIENT)
                if sqlite3_step(statement) != SQLITE_DONE {
                    let errorMessage = String(cString: sqlite3_errmsg(db))
                    throw DatabaseError.error(message: errorMessage)
                }
            } else {
                let errorMessage = String(cString: sqlite3_errmsg(db))
                throw DatabaseError.error(message: errorMessage)
            }
        }.value
    }

    func removeItem(_ key: String) async throws {
        try await Task {
            let removeQuery = "DELETE FROM Items WHERE Key = ?;"
            var statement: OpaquePointer?
            if sqlite3_prepare_v2(db, removeQuery, -1, &statement, nil) == SQLITE_OK {
                defer { sqlite3_finalize(statement) }
                sqlite3_bind_text(statement, 1, key, -1, SQLITE_TRANSIENT)
                if sqlite3_step(statement) != SQLITE_DONE {
                    let errorMessage = String(cString: sqlite3_errmsg(db))
                    throw DatabaseError.error(message: errorMessage)
                }
            } else {
                let errorMessage = String(cString: sqlite3_errmsg(db))
                throw DatabaseError.error(message: errorMessage)
            }
        }.value
    }

    func values(keyPrefix: String) async throws -> [String: String] {
        try await Task {
            var values: [String: String] = [:]
            var query: String
            var statement: OpaquePointer?
            if keyPrefix.isEmpty {
                query = "SELECT Key, Value FROM Items ORDER BY Key;"
                if sqlite3_prepare_v2(db, query, -1, &statement, nil) != SQLITE_OK {
                    let errorMessage = String(cString: sqlite3_errmsg(db))
                    throw DatabaseError.error(message: errorMessage)
                }
            } else {
                query = "SELECT Key, Value FROM Items WHERE Key LIKE ? ORDER BY Key;"
                if sqlite3_prepare_v2(db, query, -1, &statement, nil) != SQLITE_OK {
                    let errorMessage = String(cString: sqlite3_errmsg(db))
                    throw DatabaseError.error(message: errorMessage)
                }
                sqlite3_bind_text(statement, 1, "\(keyPrefix)%", -1, SQLITE_TRANSIENT)
            }
            defer { sqlite3_finalize(statement) }
            while sqlite3_step(statement) == SQLITE_ROW {
                if let keyCString = sqlite3_column_text(statement, 0),
                   let valueCString = sqlite3_column_text(statement, 1) {
                    let key = String(cString: keyCString)
                    let value = String(cString: valueCString)
                    values[key] = value
                }
            }
            return values
        }.value
    }
}

