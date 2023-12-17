import qqselfCoreLib

enum EventType {
    // Init
    struct InitStarted {}
    struct InitSucceeded { let cryptor: Cryptor }
    struct InitErrored { let error: Error }
    // Auth
    struct AuthLoginStarted { let keys: String }
    struct AuthLoginNotAuthenticated {}
    struct AuthLoginSucceeded { let cryptor: Cryptor }
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
        if (!key.starts(with: "ViewUpdate")) {
            info("Event \(key)")
        }
        let handlers = eventHandlers[key] ?? []
        for handler in handlers {
            await withCheckedContinuation { continuation in
                handler(event)
                continuation.resume()
            }
        }
    }
}
