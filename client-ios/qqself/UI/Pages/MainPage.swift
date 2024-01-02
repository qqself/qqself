import SwiftUI
import qqselfCoreLib

enum RenderMode {
  case loading
  case login
  case progress
  case register
}

@Observable class Model {
  let store: Store
  var subs: [() -> Void] = []

  var renderMode: RenderMode = .loading
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
          self.renderMode = .login
        }))
    await store.dispatch(EventType.InitStarted())
  }

  deinit {
    for v in subs { v() }
  }
}

struct MainPage: View {
  @State var model = Model()

  var body: some View {
    if model.errorMsg == nil {
      HStack {
        if model.renderMode == .loading {
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
