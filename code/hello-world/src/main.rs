mod math;

use math::prelude::*;

use rand::random;
use std::iter::from_fn;

fn main() {
    let fib = get_n_fibonacci(10);
    println!("{:?}", fib);

    let fib_range = get_fibonacci_in_range(5..=10);
    println!("{:?}", fib_range);    

    let primes = get_primes_up_to(10);
    println!("{:?}", primes);

    let primes_range = get_primes_in_range(10..=20);
    println!("{:?}", primes_range);

    //let is_seven_a_prime = math::primes::is_prime(7);

    let random_numbers: Vec<u32> = from_fn(|| Some(random::<u32>())).take(5).collect();
    println!("{:?}", random_numbers);
}
