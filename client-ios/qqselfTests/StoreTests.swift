import XCTest
import qqselfCoreLib

class OfflineApi: APIProvider {
  func set(payload: String) async throws -> String {
    throw ApiError.networkError
  }

  func find(payload: String) async throws -> [EncryptedEntry] {
    throw ApiError.networkError
  }

  func deleteAccount(payload: String) async throws {
    throw ApiError.networkError
  }
}

class StoreTests: XCTestCase {

  func testDispatchSubscribe() async {
    let store = Store(api: OfflineApi())
    var initialized = false
    let unsubscribe = store.subscribe(
      EventType.InitSucceeded.self,
      handler: { event in
        initialized = true
      })
    await store.dispatch(EventType.InitStarted())
    XCTAssert(initialized)
    unsubscribe()
  }

}
