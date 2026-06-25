pub fn get_primes_up_to(n: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    for num in 2..=n {
        if is_prime(num) {
            primes.push(num);
        }
    }
    primes
}

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn get_primes_in_range(range: std::ops::RangeInclusive<u32>) -> Vec<u32> {
    let mut primes = Vec::new();
    for num in range {
        if is_prime(num) {
            primes.push(num);
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_primes_up_to() {
        let primes = get_primes_up_to(10);
        assert_eq!(primes, vec![2, 3, 5, 7]);
    }   

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
    }

    #[test]
    fn test_get_primes_in_range() {
        let primes = get_primes_in_range(10..=20);
        assert_eq!(primes, vec![11, 13, 17, 19]);
    }
}