use integer_sqrt::IntegerSquareRoot;

#[cfg(test)]
mod tests {
    use crate::{check_prime, check_area};

    #[test]
    fn it_works() {
        for i in 0..20 {
            println!("{} is a prime number: {}", i, check_prime(i));
        }
        println!("Prime numbers from 1 to 20: {:?}", check_area(1, 20));
        println!("It works!");
    }
}

pub fn check_prime(number:u32) -> bool {
    if number < 1 { return false }
    for divisor in 2..number.integer_sqrt()+1 {
        if number%divisor == 0 { return false }
    }
    true
}

pub fn check_area(start:u32, end:u32) -> Vec<u32> {
    let mut primes:Vec<u32> = Vec::new();
    let mut divisor:u32;
    if start < 1 && start >= end { return primes }
    for number in start..end {
        divisor = 2;
        while (number%divisor != 0) && (divisor <= number.integer_sqrt()) { divisor = divisor + 1; }
        if (divisor >= number.integer_sqrt()+1) && (divisor != 1) { primes.push(number); }
    }
    primes
}
