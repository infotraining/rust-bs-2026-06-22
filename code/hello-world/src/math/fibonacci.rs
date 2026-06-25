pub fn get_n_fibonacci(n: u32) -> Vec<u32> {
    match n {
        0 => vec![0],
        1 => vec![0, 1],    
        _ => {
            let mut fib = get_n_fibonacci(n - 1);
            let next = fib[fib.len() - 1] + fib[fib.len() - 2];
            fib.push(next);
            fib
        }
    }
}

pub struct FibonacciSequence {
    current: u32,
    next: u32,
}

impl Iterator for FibonacciSequence {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current + self.next;
        let current = self.current;
        self.current = self.next;
        self.next = new_next;

        Some(current)
    }
}

pub fn fibonacci_sequence() -> FibonacciSequence {
    FibonacciSequence { current: 0, next: 1 }
}

pub fn get_fibonacci_in_range(range: std::ops::RangeInclusive<u32>) -> Vec<u32> {
    let sequence = fibonacci_sequence();

    sequence.skip_while(|n| n < range.start()).take_while(|n| n <= range.end()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_n_fibonacci() {
        let fib = get_n_fibonacci(10);
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55]); 
    }

    #[test]
    fn test_fibonacci_sequence() {
        let mut fib_seq = fibonacci_sequence();
        assert_eq!(fib_seq.next(), Some(0));
        assert_eq!(fib_seq.next(), Some(1));
        assert_eq!(fib_seq.next(), Some(1));
        assert_eq!(fib_seq.next(), Some(2));
        assert_eq!(fib_seq.next(), Some(3));
        assert_eq!(fib_seq.next(), Some(5));
        assert_eq!(fib_seq.next(), Some(8));
    }

    #[test]
    fn test_fibonacci_sequence_with_adaptors() {
        let fib_seq = fibonacci_sequence();
        let fib_up_to_10: Vec<u32> = fib_seq.take_while(|&x| x <= 10).collect();
        assert_eq!(fib_up_to_10, vec![0, 1, 1, 2, 3, 5, 8]);
    }

    #[test]
    fn test_get_fibonacci_in_range() {
        let fib = get_fibonacci_in_range(5..=10);
        assert_eq!(fib, vec![5, 8]);

        assert_eq!(get_fibonacci_in_range(5..=100), vec![5, 8, 13, 21, 34, 55, 89]);
    }
}