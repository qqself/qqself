// Panics from Rust are reported via console_error_panic_hook which then calls
// console.error. During test we consider any console.error call as a test failure
// and fail immediately
console.error = function (args) {
  const err = JSON.stringify(args).replaceAll("\\n", "\n")
  throw new Error(err)
}
