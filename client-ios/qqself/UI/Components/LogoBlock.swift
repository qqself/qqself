import SwiftUI
import qqselfCoreLib

struct LogoBlock<Content: View>: View {
    let content: Content
        
    init(@ViewBuilder content: @escaping () -> Content) {
        self.content = content()
    }
    
    let offset = 25.0
    var body: some View {
        VStack( ) {
            Spacer().frame(height: offset)
            Image("logo_512")
                .resizable()
                .aspectRatio(contentMode: .fit)
            Spacer().frame(height: offset)
            self.content
        }
        .padding()
    }
}

#Preview {
    LogoBlock { Text("hello") }
}


