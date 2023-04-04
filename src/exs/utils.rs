use std::{
    io::{self, Write},
    str::FromStr,
};

/// Prints the msg without a new line and flushes out the stdin
/// # Arguments
/// * `msg` - A string slice that is displayed as the message.
pub fn printwoln(msg: &str) {
    print!("{}", msg);
    io::stdout().flush().unwrap();
}

/// Generic input function to get user input from the command line.
/// # Example
///
/// ```
/// let x: u32 = exs::input().expect("Entered value is not an integer");
/// ```
pub fn input<T: FromStr>() -> Result<T, T::Err> {
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Error reading the input");

    let x = x.trim().parse::<T>();
    x
}
