mod primes;
mod fibonacci;

pub mod prelude {
    pub use super::primes::{get_primes_in_range, get_primes_up_to};
    pub use super::fibonacci::*;
}