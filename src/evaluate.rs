#[path = "display_error.rs"]
mod display_error;
pub fn tok(tokens: Vec<(String,bool)>,index: usize,mut istrue: bool) -> bool {
