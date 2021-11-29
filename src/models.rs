use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    author: String,
    title: String,
    published_in: u16,
}
