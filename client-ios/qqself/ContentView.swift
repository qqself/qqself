import SwiftUI
import qqselfCoreLib

struct ContentView: View {
    let hash = stringHash(input: "Hello")
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Rust::StableHash('Hello') = " + hash)
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
