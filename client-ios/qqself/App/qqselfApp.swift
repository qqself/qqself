import SwiftUI
import qqselfCoreLib

@main
struct qqselfApp: App {
    var body: some Scene {
        WindowGroup {
            MainView().onAppear {
                info(buildInfo())
            }
        }
    }
}
