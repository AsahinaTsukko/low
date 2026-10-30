use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    id: usize,
    model: String,
}
