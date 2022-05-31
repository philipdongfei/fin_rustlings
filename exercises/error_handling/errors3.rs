// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!


use std::num::ParseIntError;

fn main() /*-> Result<(), ParseIntError>*/ {
    let mut tokens = 100;
    let pretend_user_input = "8";

    
    //let cost = total_cost(pretend_user_input)?; // modify main return Result<(), ParseIntError> 
    /*
    let cost = match total_cost(pretend_user_input) {
        Ok(v) => v,
        Err(e) => panic!("total_cost error: {:?}", e),
    };
    */
    ///or
    let cost = total_cost(pretend_user_input).unwrap_or_else(|e| panic!("total cost error: {:?}", e));
    ///////////

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    //Ok(()) // 15 add ?
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
