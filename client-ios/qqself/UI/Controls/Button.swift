
import SwiftUI

struct PrimaryButton: View {
    let text: String
    let action: () -> Void

    init(_ text: String, action: @escaping () -> Void) {
        self.text = text
        self.action = action
    }

    var body: some View {
        Button(action: action, label: {
            Text(text)
        })
        .padding()
        .background(Color.black)
        .foregroundColor(.white)
        .cornerRadius(8)
    }
}

#Preview {
    PrimaryButton("Custom button") {
        print("Button tapped")
    }
}
