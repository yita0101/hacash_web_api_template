

// We need the trait in scope to use Utc::timestamp().
use chrono::{TimeZone, Utc, Duration};

use wasm_bindgen::prelude::*;

use crate::core::account::Account;
use crate::core::field::Address;
use crate::core::field::Amount;



/******** wasm_api ********/

macro_rules! or_return {
    ($tip:expr, $gain:expr) => (
        match $gain {
            Ok(obj) => obj,
            Err(e) => {
                return format!("[ERROR] {}: {}", $tip, e)
            }
        }
    )
}


include!{"amount.rs"}
include!{"account.rs"}
include!{"sign.rs"}

