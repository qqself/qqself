import XCTest
import qqselfCoreLib

// HACK: XCTAssertEqual(try await storage.getItem("foo")) doesn't compile.
//       Prior binding to variable is needed

func storages(clear: Bool = true, dbName: String = "test") async -> [Storage] {
  // Return both implementation of a storage to ensure both works
  let storage1 = try! newStorage(dbName: dbName, persistent: false)
  let storage2 = try! newStorage(dbName: dbName, persistent: true)
  if clear {
    try! await storage1.clear()
    try! await storage2.clear()
  }
  return [storage1, storage2]
}

class StorageTests: XCTestCase {

  func testGetItemSetItem() async throws {
    for storage in await storages() {
      var val = try await storage.getItem("foo")
      XCTAssertNil(val)
      try await storage.setItem(key: "foo", value: "bar")
      val = try await storage.getItem("foo")
      XCTAssertEqual(val, "bar")
    }
  }

  func testValues() async throws {
    for storage in await storages() {
      var val = try await storage.values(keyPrefix: "")
      XCTAssertEqual(val, [:])
      let data = [
        "foo.1": "bar1",
        "foo.2": "bar3",
        "zzz.1": "bar2",
      ]
      for (key, value) in data {
        try await storage.setItem(key: key, value: value)
      }
      // Values are sorted by the key
      let sortedData = Dictionary(uniqueKeysWithValues: data.sorted { $0.key < $1.key })
      val = try await storage.values(keyPrefix: "")
      XCTAssertEqual(val, sortedData)
      // Values are sorted and filtered by the prefix
      let filteredData = Dictionary(
        uniqueKeysWithValues: data.filter { $0.key.hasPrefix("foo") }.sorted { $0.key < $1.key })
      val = try await storage.values(keyPrefix: "foo")
      XCTAssertEqual(val, filteredData)
    }
  }

  func testCount() async throws {
    for storage in await storages() {
      var count = try await storage.itemCount()
      XCTAssertEqual(count, 0)
      try await storage.setItem(key: "foo", value: "bar")
      count = try await storage.itemCount()
      XCTAssertEqual(count, 1)
      try await storage.setItem(key: "bar", value: "foo")
      count = try await storage.itemCount()
      XCTAssertEqual(count, 2)
    }
  }

  func testClear() async throws {
    for storage in await storages() {
      var count = try await storage.itemCount()
      XCTAssertEqual(count, 0)
      try await storage.setItem(key: "foo", value: "bar")
      count = try await storage.itemCount()
      XCTAssertEqual(count, 1)
      try await storage.clear()
      count = try await storage.itemCount()
      XCTAssertEqual(count, 0)
    }
  }

  func testSessionPersistence() async throws {
    // Both storages should persist data during the same session. If we recreated storages data should remain available
    for storage in await storages(clear: true) {
      try await storage.setItem(key: "foo", value: "bar")
    }
    for storage in await storages(clear: false) {
      let val = try await storage.getItem("foo")
      XCTAssertEqual(val, "bar")
    }
  }

  func testDBName() async throws {
    // Both storages store data separately for each dbName
    for storage in await storages(clear: true, dbName: "test1") {
      try await storage.setItem(key: "foo", value: "bar")
    }
    for storage in await storages(clear: false, dbName: "test2") {
      let val = try await storage.getItem("foo")
      XCTAssertEqual(val, nil)
    }
  }
}
