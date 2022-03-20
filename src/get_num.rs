#[path = "is_num.rs"]
mod is_num;
#[path = "display_error.rs"]
mod display_error;
// Get full number
pub fn find(e: Vec<String>,previous:String) -> String {
let mut num = "".to_string();
#[allow(unused_variables)]
let mut period_has_occured = false;
