// Logger similar to Rust https://docs.rs/log/latest/log/ to have consistent
// logging workflows across whole code base

import Foundation
import qqselfCoreLib

enum LogLevel: String {
  case trace = "trace"
  case debug = "debug"
  case info = "info"
  case warn = "warn"
  case error = "error"
}

private let logLevels: [LogLevel: Int] = [
  .trace: 1,
  .debug: 2,
  .info: 3,
  .warn: 4,
  .error: 5,
]

private let logLevelMinimum = logLevels[config.logLevel]!

private func log(logLevel: LogLevel, msg: String) {
  guard let levelValue = logLevels[logLevel], levelValue >= logLevelMinimum else {
    return
  }
  let level = logLevel.rawValue.uppercased().padding(toLength: 5, withPad: " ", startingAt: 0)
  let timestamp = DateFormatter.localizedString(from: Date(), dateStyle: .short, timeStyle: .medium)
  print("[\(timestamp) \(level)] \(msg)")
}

private class OnPanic: PanicHook {
  func onPanic(msg: String) {
    Log.error(msg)
  }
}

func setPanicHook() {
  setPanicHook(hook: OnPanic())
}

struct Log {
  static func error(_ msg: String) { log(logLevel: .error, msg: msg) }
  static func warn(_ msg: String) { log(logLevel: .warn, msg: msg) }
  static func info(_ msg: String) { log(logLevel: .info, msg: msg) }
  static func debug(_ msg: String) { log(logLevel: .debug, msg: msg) }
  static func trace(_ msg: String) { log(logLevel: .trace, msg: msg) }
}
