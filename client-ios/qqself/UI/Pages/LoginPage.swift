import SwiftUI
import MobileCoreServices
import UniformTypeIdentifiers
import qqselfCoreLib

// TODO Right key file name is `qqself.keys`, but using such name and UTType
//      file selection didn't work for some reason in simulator - investigate
struct LoginPage: View {
    @State var selectedKeyFile: URL?
    @State var newGeneratedKeyFile: URL?
    @State var showDocumentPickerReader: Bool = false
    @State var showDocumentPickerWriter: Bool = false
    @State var errorMsg: String?

    func onLoginTapped() {
        showDocumentPickerReader = true
    }

    func onRegisterTapped() {
        let documentDirectory = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!
        let filePath = documentDirectory.appendingPathComponent("qqself-keys.txt")

        do {
            let newKeys = cryptorGenerateNew()
            let text = newKeys.serializeKeys()
            try text.write(to: filePath, atomically: true, encoding: .utf8)
            info("Generated new key \(newKeys.publicKeyHash()) to \(filePath)")
        } catch {
            errorMsg = "Error while generating new key: \(error)"
            return
        }
        newGeneratedKeyFile = filePath
        showDocumentPickerWriter = true
    }

    var body: some View {
        if errorMsg == nil {
            VStack {
                LogoBlock {
                    Spacer()
                    PrimaryButton("Login with key file", action: onLoginTapped)
                    PrimaryButton("Register", action: onRegisterTapped)
                }
            }
            .sheet(isPresented: $showDocumentPickerReader, onDismiss: {
                print("Reader done")
            }, content: {
                DocumentPicker(selectedFile: $selectedKeyFile, mode: .Reader)
            })
            .sheet(isPresented: $showDocumentPickerWriter, onDismiss: {
                print("Writer done")
            }, content: {
                DocumentPicker(selectedFile: $newGeneratedKeyFile, mode: .Writer)
            })
        } else {
            Text("Error: \(errorMsg!)")
            PrimaryButton("Close", action: {
                errorMsg = nil
            })
        }
    }
}

enum DocumentPickerMode {
    case Reader
    case Writer
}
struct DocumentPicker: UIViewControllerRepresentable {
    @Binding var selectedFile: URL?
    var mode: DocumentPickerMode

    func makeCoordinator() -> Coordinator {
        return Coordinator(parent: self)
    }

    func makeUIViewController(context: Context) -> UIDocumentPickerViewController {
        let documentPicker = mode == .Reader ? UIDocumentPickerViewController(forOpeningContentTypes: [.plainText], asCopy: true) :
            UIDocumentPickerViewController(forExporting: [selectedFile!], asCopy: false)
        documentPicker.delegate = context.coordinator
        return documentPicker
    }

    func updateUIViewController(_ uiViewController: UIDocumentPickerViewController, context: Context) {}
    
    class Coordinator: NSObject, UIDocumentPickerDelegate {
        var parent: DocumentPicker

        init(parent: DocumentPicker) {
            self.parent = parent
        }

        func documentPicker(_ controller: UIDocumentPickerViewController, didPickDocumentsAt urls: [URL]) {
            parent.selectedFile = urls.first
        }
    }
}

#Preview {
    LoginPage()
}

#Preview("Error") {
    LoginPage(errorMsg: "No disk left on a device")
}
