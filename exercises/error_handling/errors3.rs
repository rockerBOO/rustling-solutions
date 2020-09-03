// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    match total_cost(pretend_user_input) {
        Err(err) => println!("{}", err),
        Ok(cost) if cost > tokens => println!("You can't afford that many!"),
        Ok(cost) => println!("you now have {} tokens.", tokens),
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    match item_quantity.parse::<i32>() {
        Err(err) => Err(err),
        Ok(qty) => Ok(qty * cost_per_item + processing_fee),
    }
}
