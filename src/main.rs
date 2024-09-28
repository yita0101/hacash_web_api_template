
#[macro_use]
extern crate ini;
#[macro_use]
extern crate lazy_static; 

mod x16rs;

#[macro_use]
mod sys;
#[macro_use]
mod base;
mod config;
mod interface;
#[macro_use]
mod core;
#[macro_use]
mod protocol;
mod mint;
#[macro_use]
mod vm;

// mod chain;
// mod node;
// mod server;
// mod tests;

// pub mod run;

// use crate::run::*;

mod wasm_api;

use crate::wasm_api::hac_to_mei;


// wasm-pack build --target web
fn main() {
   let string = hac_to_mei(String::from("120:245"));
    println!("{}", string)
}









