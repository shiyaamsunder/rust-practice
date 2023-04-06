#![allow(dead_code)]
pub struct Ex1 {}

impl Ex1 {
    pub fn odd_or_even(x: &u32) -> String {
        let result = String::from("Odd");
        if x % 2 == 0 {
            let result = String::from("Even");
            result
        } else {
            result
        }
    }

    pub fn fibonacci(n: i32) -> Result<Vec<u32>, &'static str> {
        if n < 0 {
            return Err("Cannot be negatice");
        } else if n == 0 {
            return Err("Zero is not a valid argument");
        } else if n == 1 {
            return Ok(vec![0]);
        }
        let mut result: Vec<u32> = vec![];
        let mut i = 0;
        let mut a = 0;
        let mut b = 1;
        result.push(a);
        result.push(b);

        for _i in 1..n {}
        while i <= n {
            let c = a + b;
            result.push(c);
            a = b;
            b = c;
            i += 1;
        }
        Ok(result)
    }

    pub fn armstrong(n: i32) -> String {
        let mut result = String::from("Not an armstong number");

        let mut digits = 0;
        let mut number = n;
        let mut remainder: i32;

        let mut r = 0;

        while number != 0 {
            number = number / 10;
            digits += 1;
        }

        number = n;

        while number != 0 {
            remainder = number % 10;
            r += i32::pow(remainder, digits);
            number = number / 10;
        }

        if r == n {
            result = String::from("This is an armstong number");
            result
        } else {
            result
        }
    }
}
