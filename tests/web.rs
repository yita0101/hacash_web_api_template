//! Test suite for the Web and headless browsers.

// #![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use hacash_web_api::wasm_api::hac_to_mei;

wasm_bindgen_test_configure!(run_in_browser);

// wasm-pack test --headless --chrome
//the order have some problem
#[wasm_bindgen_test]
fn pass() {
    let string = hac_to_mei(String::from("120:245"));
    println!("{}", string)
}

