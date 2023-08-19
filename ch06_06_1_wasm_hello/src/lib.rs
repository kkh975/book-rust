extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// 자바스크립트 함수를 러스트에서 사용하기 위해
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// 러스트로 자바스크립에서 사용할 함수 정의
#[wasm_bindgen]
pub fn hello(name: &str) {
    let msg = format!("Hello, {}!", name);
    alert(&msg);
}

#[wasm_bindgen]
pub fn rust_mul(a: i32, b: i32) -> i32 {
    a * b
}

