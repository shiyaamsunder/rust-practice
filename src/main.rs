mod exs;

use exs::ex1;
use exs::utils;

fn main() {
    utils::printwoln("Enter a number: ");
    let x: u32 = utils::input().expect("Entered number is not an integer");
    let result = ex1::odd_or_even(&x);

    println!("Entered number is {}", result.to_lowercase());
}
