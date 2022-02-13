use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
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

    #[wasm_bindgen(setter)]
    pub fn set_uri(&mut self, uri: String) {
        self.uri = uri;
    }

    #[wasm_bindgen(getter)]
    pub fn method(&self) -> String {
        self.method.to_string()
    }

    #[wasm_bindgen(setter)]
    pub fn set_method(&mut self, method: String) {
        self.method = method;
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> String {
        self.body.clone().unwrap().to_string()
    }

    #[wasm_bindgen(setter)]
    pub fn set_body(&mut self, body: String) {
        self.body = Option::from(body);
    }

    #[wasm_bindgen(getter)]
    pub fn headers(&self) -> JsValue {
        JsValue::from_serde(&self.headers).unwrap()
    }

    pub fn headers_append(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
}

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

    #[wasm_bindgen(setter)]
    pub fn set_status_code(&mut self, status_code: String) {
        self.status_code = status_code;
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> Option<String> {
        self.body.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_body(&mut self, body: String) {
        self.body = Some(body);
    }

    #[wasm_bindgen(getter)]
    pub fn headers(&self) -> JsValue {
        JsValue::from_serde(&self.headers).unwrap()
    }
}