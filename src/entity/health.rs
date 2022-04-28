use serde::{Serialize};

#[derive(Serialize)]
pub struct Health {
    pub status: String,
}