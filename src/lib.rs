//! JavaScript Adapter
//!
//! HTTP messages are a good way to exchange data across the Internet, why not use them to exchange
//! data between Rust/WASM and JavaScript.
//!
use std::collections::HashMap;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
///
/// Represents a request originating from JavaScript.
/// ```javascript
/// // JavaScript Example
/// import {JsRequest} from "my-wasm-app"
///
/// const jsRequest = new JsRequest("https://www.rust-lang.org/", "GET");
/// jsRequest.headers_append("Content-Type", "text/html");
///
/// // pass request to WASM app
/// ```
///
/// Consume the request in Rust
/// ```
/// use javascript_adapter::JsRequest;
///
/// #[wasm_bindgen]
/// pub fn app(js_request: JsRequest) {
///     // do things with js_request
/// }
///
pub struct JsRequest {
    #[wasm_bindgen(skip)]
    pub uri: String,
    #[wasm_bindgen(skip)]
    pub method: String,
    #[wasm_bindgen(skip)]
    pub body: Option<String>,

    #[wasm_bindgen(skip)]
    pub headers: HashMap<String, String>,
}

#[wasm_bindgen]
impl JsRequest {
    #[wasm_bindgen(constructor)]
    pub fn new(uri: String, method: String) -> JsRequest {
        JsRequest {
            uri: uri.into(),
            method: method.into(),
            body: None,
            headers: Default::default(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn uri(&self) -> String {
        self.uri.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn method(&self) -> String {
        self.method.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> String {
        self.body.clone().unwrap().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn headers(&self) -> JsValue {
        JsValue::from_serde(&self.headers).unwrap()
    }

    pub fn headers_append(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
}

/// Represents a response originating from Rust.
///
/// ```
/// use crate::javascript_adapter::JsResponse;
///
/// #[wasm_bindgen]
/// pub fn app() -> JsResponse {
///     let mut  response = JsResponse::new();
///     response.body = Some(String::from("hello world"));
///     response.headers.insert(String::from("Content-Type"), String::from("text/plain"));
///     response.status_code = String::from("200");
///
///     response
/// }
/// ```
#[wasm_bindgen]
pub struct JsResponse {
    #[wasm_bindgen(skip)]
    pub status_code: String,
    #[wasm_bindgen(skip)]
    pub headers: HashMap<String, String>,
    #[wasm_bindgen(skip)]
    pub body: Option<String>,
}


#[wasm_bindgen]
impl JsResponse {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsResponse {
        JsResponse {
            status_code: "".to_string(),
            headers: Default::default(),
            body: None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn status_code(&self) -> String {
        self.status_code.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> Option<String> {
        self.body.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn headers(&self) -> JsValue {
        JsValue::from_serde(&self.headers).unwrap()
    }
}