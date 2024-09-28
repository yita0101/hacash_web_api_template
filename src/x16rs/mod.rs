use sha3::{Digest, Sha3_256};
use sha2::{Sha256};
use ripemd::{Ripemd160};

pub const HASH_SIZE: usize = 32;

include!("x16rs.rs");
include!("hash.rs");
include!("diamond.rs");
include!("block.rs");


