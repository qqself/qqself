// Panic handler - in case of panic we want to send error logs to the calling side
// Similar to WASM console_error_panic_hook

use std::{panic, sync::Mutex};

static SAVED_HOOK: Mutex<Option<Box<dyn PanicHook>>> = Mutex::new(None);

pub trait PanicHook: Send + Sync {
    fn on_panic(&self, message: String);
}

pub fn set_panic_hook(hook: Box<dyn PanicHook>) {
    let mut guard = SAVED_HOOK.lock().unwrap();
    if guard.is_some() {
        return; // Hook was already set
    }
    // Store hook in static variable to make it accessible in panic::set_hook
    *guard = Some(hook);
    panic::set_hook(Box::new(|info: &panic::PanicInfo| {
        let hook = SAVED_HOOK.lock().unwrap();
        hook.as_ref().unwrap().on_panic(info.to_string());
    }));
}
