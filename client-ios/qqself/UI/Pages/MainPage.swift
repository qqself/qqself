import SwiftUI
import qqselfCoreLib

enum RenderMode {
  case Loading
  case Login
  case Progress
  case Register
}

@Observable class Model {
  let store: Store
  var subs: [() -> Void] = []

  var renderMode: RenderMode = .Loading
  var errorMsg: String?

  init() {
    store = Store(api: ServerApi(basePath: config.apiBasePath))
  }

  func onAppear() async {
    guard subs.isEmpty else { return }

    subs.append(
      store.subscribe(
        EventType.InitErrored.self,
        handler: { event in
          self.errorMsg = event.error.localizedDescription
        }))
    subs.append(
      store.subscribe(
        EventType.AuthLoginNotAuthenticated.self,
        handler: { _ in
          self.renderMode = .Login
        }))
    await store.dispatch(EventType.InitStarted())
  }

  deinit {
    subs.forEach { $0() }
  }
}

struct MainPage: View {
  @State var model = Model()

  var body: some View {
    if model.errorMsg == nil {
      HStack {
        if model.renderMode == .Loading {
          Text("Loading...")
        } else {
          Text("Please login!")
        }
      }
      .onAppear(perform: onAppear)
    } else {
      Text("Error: \(model.errorMsg!)")
    }
  }

  func onAppear() {
    Task {
      await model.onAppear()
    }
  }
}

#Preview {
  MainPage()
}
