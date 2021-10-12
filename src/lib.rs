use integer_sqrt::IntegerSquareRoot;

#[cfg(test)]
mod tests {
    use crate::{check_prime, check_area};

    #[test]
    fn it_works() {
        for i in 0..20 {
            println!("{} ist eine Primzahl: {}", i, check_prime(i));
        }
        println!("Primzahlen von 1'000'000 bis 1'000'100: {:?}", check_area(1000000, 1000100));
        println!("It works!");
    }
}

fn check_prime(number:u32) -> bool {
    if number < 1 { return false }
    for divisor in 2..number.integer_sqrt()+1 {
        if number%divisor == 0 { return false }
    }
    true
}

fn check_area(start:u32, end:u32) -> Vec<u32> {
    let mut primes:Vec<u32> = Vec::new();
    let mut divisor:u32;
    if start < 1 && start >= end { return primes }
    for number in start..end {
        divisor = 2;
        while number%divisor != 0 && divisor <= number.integer_sqrt()+1 { divisor = divisor + 1; }
        if divisor >= number.integer_sqrt() { primes.push(number); }
    }
    primes
}