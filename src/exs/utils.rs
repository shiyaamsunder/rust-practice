use std::{
    io::{self, Write},
    str::FromStr,
};

pub fn printwoln(msg: &str) {
    print!("{}", msg);
    io::stdout().flush().unwrap();
}

pub fn input<T: FromStr>() -> Result<T, T::Err> {
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Error reading the input");

    let x = x.trim().parse::<T>();
    x
}
