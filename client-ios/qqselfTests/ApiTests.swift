import XCTest
import qqselfCoreLib

class ApiTests: XCTestCase {
  let api = ServerApi(basePath: nil)

  func wait(seconds: TimeInterval) async throws {
    try await Task.sleep(nanoseconds: UInt64(seconds * 1_000_000_000))
  }

  func testCreateNewCryptor() {
    let cryptor = cryptorGenerateNew()
    XCTAssertNotNil(cryptor)
  }

  func testAPI() async throws {
    // First find call no data
    let cryptor = cryptorGenerateNew()
    let lines = try await api.find(payload: cryptor.signFindToken(lastId: nil))
    XCTAssertEqual(lines.count, 0)

    // Add a couple of messages
    let _ = try await api.set(payload: cryptor.encrypt(data: "msg1"))
    let _ = try await api.set(payload: cryptor.encrypt(data: "msg2"))

    // Get all messages back
    let got = try await api.find(payload: cryptor.signFindToken(lastId: nil))
    let entries = try got.map { entry in
      try cryptor.decrypt(data: entry.payload)
    }
    XCTAssertEqual(entries.sorted(), ["msg1", "msg2"])

    // Wait a bit, add a message with a new timestamp, and ensure the filter works
    try await wait(seconds: 2)
    let msgId = try await api.set(payload: cryptor.encrypt(data: "msg3"))
    try await wait(seconds: 2)
    let _ = try await api.set(payload: cryptor.encrypt(data: "msg4"))
    let filtered = try await api.find(payload: cryptor.signFindToken(lastId: msgId))
    let filteredEntries = try filtered.map { entry in
      try cryptor.decrypt(data: entry.payload)
    }
    XCTAssertEqual(filteredEntries.sorted(), ["msg4"])

    // Delete it all
    try await api.deleteAccount(payload: cryptor.signDeleteToken())
  }
}
