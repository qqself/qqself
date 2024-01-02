import Foundation

struct Config {
  let isDataPersistent: Bool
  let logLevel: LogLevel
  let apiBasePath: String
}

var config = {
  if isRunningTests() {
    return testConfig()
  }
  #if DEBUG
    return debugConfig()
  #else
    fatalError("No config available")
  #endif
}()

private func isRunningTests() -> Bool {
  return ProcessInfo.processInfo.environment["XCTestConfigurationFilePath"] != nil
}

private func testConfig() -> Config {
  return Config(
    isDataPersistent: false,
    logLevel: LogLevel.info,
    apiBasePath: "https://api.qqself.com"
  )
}

private func debugConfig() -> Config {
  return Config(
    isDataPersistent: false,
    logLevel: LogLevel.info,
    apiBasePath: "https://api.qqself.com"
  )
}
