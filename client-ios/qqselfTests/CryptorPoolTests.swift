import XCTest
import qqselfCoreLib

func newCryptorPool() async throws -> CryptorPool {
    let keys = await CryptorPool.generateKeys()
    return try CryptorPool.fromKeys(keys)
}

class CrtyptorPoolTests: XCTestCase {
    func testGenerateKeys() async throws {
        let keys = await CryptorPool.generateKeys()
        XCTAssertNotNil(keys)
    }
    
    func testSign() async throws {
        let pool = try await newCryptorPool()
        let findSync = try await pool.sign(data: .find(lastSyncId: "foo"))
        XCTAssertGreaterThan(findSync.count, 1)
        let findNoSync = try await pool.sign(data: .find(lastSyncId: nil))
        XCTAssertGreaterThan(findNoSync.count, 1)
        let delete = try await pool.sign(data: .delete)
        XCTAssertGreaterThan(delete.count, 1)
    }
    
    func testEncryptDecrypt() async throws {
        let pool = try await newCryptorPool()
        let msg = "Hello World 🎄"
        let encrypted = try await pool.encrypt(data: msg)
        XCTAssertGreaterThan(encrypted.count, 1)
        let decrypted = try await pool.decrypt(data: encrypted)
        XCTAssertEqual(decrypted, msg)
    }
}
