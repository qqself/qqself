// Logger similar to Rust https://docs.rs/log/latest/log/ to have consistent
// logging workflows across whole code base

import Foundation

enum LogLevel: String {
    case trace = "trace"
    case debug = "debug"
    case info = "info"
    case warn = "warn"
    case error = "error"
}

let logLevels: [LogLevel: Int] = [
    .trace: 1,
    .debug: 2,
    .info: 3,
    .warn: 4,
    .error: 5
]

// TODO Get level from the environment
let logLevelMinimum = logLevels[.info]!

func log(logLevel: LogLevel, msg: String) {
    guard let levelValue = logLevels[logLevel], levelValue >= logLevelMinimum else {
        return
    }
    let level = logLevel.rawValue.uppercased().padding(toLength: 5, withPad: " ", startingAt: 0)
    let timestamp = DateFormatter.localizedString(from: Date(), dateStyle: .short, timeStyle: .medium)
    print("[\(timestamp) \(level)] \(msg)")
}

let error = { log(logLevel: .error, msg: $0) }
let warn = { log(logLevel: .warn, msg: $0) }
let info = { log(logLevel: .info, msg: $0) }
let debug = { log(logLevel: .debug, msg: $0) }
let trace = { log(logLevel: .trace, msg: $0) } 
