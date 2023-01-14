use std::cmp::max;
use std::collections::HashSet;

pub fn fibonacci_memo(n: u32, memo: &mut Vec<u32>) -> u32 {
    if memo.len() > n as usize {
        return memo[n as usize];
    }
    let result = fibonacci_memo(n - 1, memo) + fibonacci_memo(n - 2, memo);
    memo.push(result);
    result
}

pub fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    let chars = s.chars();
    let end_len = s.len() / 2;
    let front_end = chars.clone().take(end_len);
    let back_end = chars.rev().take(end_len);
    for (first, last) in std::iter::zip(front_end, back_end) {
        if first != last {
            return false;
        }
    }
    true
}

pub(crate) struct PrimeMemo {
    ordered_complete_prime_set: Vec<u64>,
    complete_through: u64,
    known_prime_not_complete: HashSet<u64>,
    known_composite_not_complete: HashSet<u64>,
}

impl PrimeMemo {
    pub fn new() -> PrimeMemo {
        PrimeMemo {
            ordered_complete_prime_set: vec![2],
            complete_through: 2,
            known_prime_not_complete: HashSet::new(),
            known_composite_not_complete: HashSet::new(),
        }
    }

    fn expand_complete_set_through(&mut self, n: u64) {
        if n <= self.complete_through {
            return;
        }
        let mut next = self.complete_through + 1;
        while next <= n {
            if self.known_prime_not_complete.contains(&next) {
                self.known_prime_not_complete.remove(&next);
                self.ordered_complete_prime_set.push(next);
            } else if self.known_composite_not_complete.contains(&next) {
                self.known_composite_not_complete.remove(&next);
            } else {
                let mut is_prime = true;
                for prime in &self.ordered_complete_prime_set {
                    if prime * prime > next {
                        break;
                    }
                    if next % prime == 0 {
                        is_prime = false;
                        break;
                    }
                }
                if is_prime {
                    self.ordered_complete_prime_set.push(next);
                }
            }
            self.complete_through = next;
            next += 1;
        }
    }
    fn expand_complete_set_to_size(&mut self, n: usize) {
        if self.ordered_complete_prime_set.len() >= n {
            return;
        }
        let mut next = self.complete_through + 1;
        while self.ordered_complete_prime_set.len() < n {
            self.expand_complete_set_through(next);
            next += 1;
        }
    }
    fn expand_complete_set_to_sqrt(&mut self, n: u64) {
        while self.complete_through * self.complete_through < n {
            self.expand_complete_set_through(self.complete_through + 1);
        }
    }

    pub fn is_prime(&mut self, n: u64) -> bool {
        if n <= self.complete_through {
            return self.ordered_complete_prime_set.binary_search(&n).is_ok();
        }
        if self.known_prime_not_complete.contains(&n) {
            return true;
        }
        if self.known_composite_not_complete.contains(&n) {
            return false;
        }
        self.expand_complete_set_to_sqrt(n);
        for prime in &self.ordered_complete_prime_set {
            if prime * prime > n {
                break;
            }
            if n % prime == 0 {
                self.known_composite_not_complete.insert(n);
                return false;
            }
        }
        self.known_prime_not_complete.insert(n);
        true
    }

    pub fn factorize(&mut self, n: u64) -> Vec<u64> {
        if n == 1 {
            return vec![1];
        }
        if self.is_prime(n) {
            // this expands complete set to sqrt(n)
            return vec![1, n];
        }
        let mut result = vec![1];
        let mut remaining = n;
        let mut ruled_out_in_complete_set = 0;
        while remaining > 1 {
            let mut found = false;
            for (prime, prime_index) in self
                .ordered_complete_prime_set
                .iter()
                .zip(1..)
                .skip(ruled_out_in_complete_set)
            {
                if prime * prime > remaining {
                    break;
                }
                if remaining % prime == 0 {
                    ruled_out_in_complete_set = max(prime_index - 1, 0);
                    result.push(*prime);
                    remaining /= prime;
                    found = true;
                    break;
                }
            }
            if !found {
                result.push(remaining);
                break;
            }
        }
        result
    }

    pub fn nth_prime(&mut self, n: usize) -> u64 {
        self.expand_complete_set_to_size(n);
        self.ordered_complete_prime_set[n - 1]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_factorize() {
        let mut prime_memo = super::PrimeMemo::new();
        assert_eq!(prime_memo.factorize(1), vec![1]);
        assert_eq!(prime_memo.factorize(2), vec![1, 2]);
        assert_eq!(prime_memo.factorize(3), vec![1, 3]);
        assert_eq!(prime_memo.factorize(4), vec![1, 2, 2]);
        assert_eq!(prime_memo.factorize(50), vec![1, 2, 5, 5]);
        assert_eq!(prime_memo.factorize(100), vec![1, 2, 2, 5, 5]);
        assert_eq!(prime_memo.factorize(600851475143).last().unwrap(), &6857);
    }
    #[test]
    fn test_nth_prime() {
        let mut prime_memo = super::PrimeMemo::new();
        assert_eq!(prime_memo.nth_prime(1), 2);
        assert_eq!(prime_memo.nth_prime(2), 3);
        assert_eq!(prime_memo.nth_prime(3), 5);
        assert_eq!(prime_memo.nth_prime(4), 7);
        assert_eq!(prime_memo.nth_prime(5), 11);
        assert_eq!(prime_memo.nth_prime(6), 13);
        assert_eq!(prime_memo.nth_prime(10_001), 104743);
    }
}
