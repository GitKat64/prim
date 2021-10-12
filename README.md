###Description
Simple crate to find prime numbers.

###Example:
```
extern crate prim;

fn main() {
    println!("{}", prim::check_prime(5)); //true
    println!("{:?}", prim::check_area(1, 10)); //[2, 3, 5, 7]
}
