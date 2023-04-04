pub fn odd_or_even(x: &u32) -> String {
    let result = String::from("Odd");
    if x % 2 == 0 {
        let result = String::from("Even");
        result
    } else {
        result
    }
}
