import XCTest
import qqselfCoreLib

class StoreTests: XCTestCase {

  func testDispatchSubscribe() async {
    let store = Store()
    var cryptor: Cryptor? = nil
    let unsubscribe = store.subscribe(
      EventType.InitSucceeded.self,
      handler: { event in
        cryptor = event.cryptor
      })
    await store.dispatch(EventType.InitSucceeded(cryptor: cryptorGenerateNew()))
    XCTAssertNotNil(cryptor)
    unsubscribe()
  }

}
