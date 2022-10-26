#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioEncoder , typescript_type = "AudioEncoder")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioEncoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AudioEncoder;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CodecState")]
    # [wasm_bindgen (structural , method , getter , js_class = "AudioEncoder" , js_name = state)]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoder`, `CodecState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn state(this: &AudioEncoder) -> CodecState;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "AudioEncoder" , js_name = encodeQueueSize)]
    #[doc = "Getter for the `encodeQueueSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/encodeQueueSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn encode_queue_size(this: &AudioEncoder) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioEncoderInit")]
    #[wasm_bindgen(catch, constructor, js_class = "AudioEncoder")]
    #[doc = "The `new AudioEncoder(..)` constructor, creating a new instance of `AudioEncoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/AudioEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoder`, `AudioEncoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(init: &AudioEncoderInit) -> Result<AudioEncoder, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "AudioEncoder" , js_name = close)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn close(this: &AudioEncoder);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioEncoderConfig")]
    # [wasm_bindgen (method , structural , js_class = "AudioEncoder" , js_name = configure)]
    #[doc = "The `configure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/configure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoder`, `AudioEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn configure(this: &AudioEncoder, config: &AudioEncoderConfig);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioData")]
    # [wasm_bindgen (method , structural , js_class = "AudioEncoder" , js_name = encode)]
    #[doc = "The `encode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/encode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`, `AudioEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn encode(this: &AudioEncoder, data: &AudioData);
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "AudioEncoder" , js_name = flush)]
    #[doc = "The `flush()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/flush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn flush(this: &AudioEncoder) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioEncoderConfig")]
    # [wasm_bindgen (static_method_of = AudioEncoder , js_class = "AudioEncoder" , js_name = isConfigSupported)]
    #[doc = "The `isConfigSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/isConfigSupported)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoder`, `AudioEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_config_supported(config: &AudioEncoderConfig) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "AudioEncoder" , js_name = reset)]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn reset(this: &AudioEncoder);
}