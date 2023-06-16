// Logger similar to Rust https://docs.rs/log/latest/log/ to have consistent
// logging workflows across whole code base

const logLevels = {
  trace: 0,
  debug: 1,
  info: 2,
  warn: 3,
  error: 4,
}

const logLevelDefault = logLevels.warn
const logLevelMinimum = logLevels[import.meta.env.VITE_LOG_LEVEL as LogLevel] || logLevelDefault

type LogLevel = keyof typeof logLevels

const log = (logLevel: LogLevel, msg: string) => {
  if (logLevels[logLevel] < logLevelMinimum) return
  const level = logLevel.toUpperCase().padEnd(5)
  console.log(`[${new Date().toISOString()} ${level}] ${msg}`)
}

export const error = log.bind(null, "error")
export const warn = log.bind(null, "warn")
export const info = log.bind(null, "info")
export const debug = log.bind(null, "debug")
export const trace = log.bind(null, "trace")
