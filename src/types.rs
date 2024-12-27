use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct Error {
   pub data: String,
}