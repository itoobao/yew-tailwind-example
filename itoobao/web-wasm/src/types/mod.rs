mod user;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub use user::*;
