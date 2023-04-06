mod exs;

use exs::ex1::Ex1;
use exs::utils::Utils;

fn main() {
    Utils::printwoln("Enter a number: ");
    let x: i32 = Utils::input().expect("Entered number is not an integer");

    // let result = Ex1::fibonacci(x).unwrap_or_else(|err| {
    //     println!("Error: {err}");
    //     process::exit(0);
    // });

    let result = Ex1::armstrong(x);

    println!("{result}");
}
