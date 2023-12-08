import XCTest
import qqselfCoreLib

func storages(clear: Bool = true, dbName: String = "test") -> [Storage] {
    // Return both implementation of a storage to ensure both works
    let storage1 = try! newStorage(dbName: dbName, persistant: false)
    let storage2 = try! newStorage(dbName: dbName, persistant: true)
    if (clear) {
        try! storage1.clear()
        try! storage2.clear()
    }
    return [storage1, storage2]
}

class StorageTests: XCTestCase {

    func testGetItemSetItem() throws {
        for storage in storages() {
            XCTAssertNil(try storage.getItem("foo"))
            try storage.setItem(key: "foo", value: "bar")
            XCTAssertEqual(try storage.getItem("foo"), "bar")
        }
    }

    func testValues() throws {
        for storage in storages() {
            XCTAssertEqual(try storage.values(keyPrefix: ""), [:])
            let data = [
                "foo.1": "bar1",
                "foo.2": "bar3",
                "zzz.1": "bar2",
            ]
            for (key, value) in data {
                try storage.setItem(key: key, value: value)
            }
            // Values are sorted by the key
            let sortedData = Dictionary(uniqueKeysWithValues: data.sorted { $0.key < $1.key })
            XCTAssertEqual(try storage.values(keyPrefix: ""), sortedData)
            // Values are sorted and filterd by the prefix
            let filteredData = Dictionary(uniqueKeysWithValues: data.filter { $0.key.hasPrefix("foo")}.sorted { $0.key < $1.key })
            XCTAssertEqual(try storage.values(keyPrefix: "foo"), filteredData)
        }
    }

    func testCount() throws {
        for storage in storages() {
            XCTAssertEqual(try storage.itemCount(), 0)
            try storage.setItem(key: "foo", value: "bar")
            XCTAssertEqual(try storage.itemCount(), 1)
            try storage.setItem(key: "bar", value: "foo")
            XCTAssertEqual(try storage.itemCount(), 2)
        }
    }

    func testClear() throws {
        for storage in storages() {
            XCTAssertEqual(try storage.itemCount(), 0)
            try storage.setItem(key: "foo", value: "bar")
            XCTAssertEqual(try storage.itemCount(), 1)
            try storage.clear()
            XCTAssertEqual(try storage.itemCount(), 0)
        }
    }

    func testSessionPersistence() throws {
        // Both storages should persist data during the same session. If we recreated storages data should remain available
        for storage in storages(clear: true) {
            try storage.setItem(key: "foo", value: "bar")
        }
        for storage in storages(clear: false) {
            XCTAssertEqual(try storage.getItem("foo"), "bar")
        }
    }
    
    func testDBName() throws {
        // Both storages store data separately for each dbName
        for storage in storages(clear: true, dbName: "test1") {
            try storage.setItem(key: "foo", value: "bar")
        }
        for storage in storages(clear: false, dbName: "test2") {
            XCTAssertEqual(try storage.getItem("foo"), nil)
        }
    }
}

