import qqselfCoreLib

enum EventType {
  // Init
  struct InitStarted {}
  struct InitSucceeded { let cryptor: CryptorPool? }
  struct InitErrored { let error: Error }
  // Auth
  struct AuthLoginStarted { let keys: String }
  struct AuthLoginNotAuthenticated {}
  struct AuthLoginSucceeded { let cryptor: CryptorPool }
  struct AuthLoginErrored { let error: Error }
  struct AuthRegistrationStarted { let mode: RegistrationMode }
  struct AuthRegistrationSucceeded { let cryptor: Cryptor }
  struct AuthLogoutStarted {}
  struct AuthLogoutSucceeded {}
}

enum RegistrationMode {
  case interactive
  case automatic
}

protocol Events {
  associatedtype T = EventType
}

class Store {
  private var eventHandlers = [String: [(Any) -> Void]]()
  private let api: APIProvider

  init(api: APIProvider) {
    self.api = api
  }

  func subscribe<E>(_ eventName: E.Type, handler: @escaping (E) -> Void) -> () -> Void {
    let key = String(describing: eventName)
    eventHandlers[key, default: []].append { event in
      if let typedEvent = event as? E {
        handler(typedEvent)
      }
    }
    return {
      self.eventHandlers[key]?.removeAll { $0 as AnyObject === handler as AnyObject }
    }
  }

  func dispatch<E>(_ event: E) async {
    let key = String(describing: type(of: event))
    Log.info("Event \(key)")

    await processEvent(event)

    let handlers = eventHandlers[key] ?? []
    for handler in handlers {
      await withCheckedContinuation { continuation in
        handler(event)
        continuation.resume()
      }
    }
  }

  func processEvent<E>(_ event: E) async {
    switch event {
    case is EventType.InitStarted:
      await Auth.onInitStarted(self)
    case let event as EventType.InitSucceeded:
      await Auth.onInitSucceeded(self, pool: event.cryptor)
    default:
      let event = String(describing: event)
      Log.warn("Unknown event type: \(event)")
      break
    }
  }
}
