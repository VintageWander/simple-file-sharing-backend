pub mod cookie;
pub mod decode;
pub mod encode;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    name: String,
    exp: usize,
}

pub enum TokenType {
    Access,
    Refresh,
}
