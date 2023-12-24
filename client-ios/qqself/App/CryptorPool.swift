import Foundation
import qqselfCoreLib

enum SignData {
  case find(lastSyncId: String?)
  case delete
}

// Wrapper for all encryption logic to ensure it runs in the background thread.
// It's CPU heavy and we don't want to ever block the main thread and UI
struct CryptorPool {

  let cryptor: Cryptor

  private init(_ cryptor: Cryptor) {
    self.cryptor = cryptor
  }

  static func fromKeys(_ keys: String) throws -> CryptorPool {
    let cryptor = try cryptorFromDeserializedKeys(data: keys)
    return CryptorPool(cryptor)
  }

  static func generateKeys() async -> String {
    await Task {
      let generatedCryptor = cryptorGenerateNew()
      return generatedCryptor.serializeKeys()
    }.value
  }

  func serialiseKeys() -> String {
    return cryptor.serializeKeys()
  }

  func sign(data: SignData) async throws -> String {
    try await Task {
      switch data {
      case .find(let lastSyncId):
        try self.cryptor.signFindToken(lastId: lastSyncId)
      case .delete:
        try self.cryptor.signDeleteToken()
      }
    }.value
  }

  func encrypt(data: String) async throws -> String {
    try await Task {
      try self.cryptor.encrypt(data: data)
    }.value
  }

  func decrypt(data: String) async throws -> String {
    try await Task {
      try self.cryptor.decrypt(data: data)
    }.value
  }

}
