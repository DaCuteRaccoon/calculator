#[path = "get_input.rs"]
mod get_input;
#[path = "find.rs"]
mod find;
#[path = "split_by_delimeters.rs"]
mod split_by_delimeters;
#[path = "solution_of.rs"]
mod solution_of;
#[path = "evaluate.rs"]
mod evaluate;
fn remove_trailing_zeros(num: String) -> String {
  let mut trunc = 0;
  for i in num.chars().rev() {
    if i == '0' {
      trunc += 1;
    } else { break; }}
  return num[0..num.len()-trunc].to_string()
}
