use serde::{ Serialize, Deserialize };
use std::collections::HashMap;

#[derive(Serialize,Deserialize)]
pub struct Request {
    pub path: String,
    pub options: HashMap< String, String >
}