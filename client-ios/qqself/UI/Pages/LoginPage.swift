import MobileCoreServices
import SwiftUI
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
    Task {
      let documentDirectory = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
        .first!
      let filePath = documentDirectory.appendingPathComponent("qqself-keys.txt")

      do {
        let newKeys = await CryptorPool.generateKeys()
        try newKeys.write(to: filePath, atomically: true, encoding: .utf8)
        Log.info("Generated new keys to \(filePath)")
      } catch {
        errorMsg = "Error while generating new key: \(error)"
        return
      }
      newGeneratedKeyFile = filePath
      showDocumentPickerWriter = true
    }
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
      .sheet(
        isPresented: $showDocumentPickerReader,
        onDismiss: {
          print("Reader done")
        },
        content: {
          DocumentPicker(selectedFile: $selectedKeyFile, mode: .reader)
        }
      )
      .sheet(
        isPresented: $showDocumentPickerWriter,
        onDismiss: {
          print("Writer done")
        },
        content: {
          DocumentPicker(selectedFile: $newGeneratedKeyFile, mode: .writer)
        })
    } else {
      Text("Error: \(errorMsg!)")
      PrimaryButton(
        "Close",
        action: {
          errorMsg = nil
        })
    }
  }
}

enum DocumentPickerMode {
  case reader
  case writer
}
struct DocumentPicker: UIViewControllerRepresentable {
  @Binding var selectedFile: URL?
  var mode: DocumentPickerMode

  func makeCoordinator() -> Coordinator {
    return Coordinator(parent: self)
  }

  func makeUIViewController(context: Context) -> UIDocumentPickerViewController {
    let documentPicker =
      mode == .reader
      ? UIDocumentPickerViewController(forOpeningContentTypes: [.plainText], asCopy: true)
      : UIDocumentPickerViewController(forExporting: [selectedFile!], asCopy: false)
    documentPicker.delegate = context.coordinator
    return documentPicker
  }

  func updateUIViewController(_ uiViewController: UIDocumentPickerViewController, context: Context)
  {}

  class Coordinator: NSObject, UIDocumentPickerDelegate {
    var parent: DocumentPicker

    init(parent: DocumentPicker) {
      self.parent = parent
    }

    func documentPicker(
      _ controller: UIDocumentPickerViewController, didPickDocumentsAt urls: [URL]
    ) {
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
