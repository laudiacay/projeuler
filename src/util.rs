use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

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
    pub(crate) ordered_complete_prime_set: Vec<u64>,
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

    pub fn expand_complete_set_through(&mut self, n: u64) {
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
    pub(crate) fn expand_complete_set_to_size(&mut self, n: usize) {
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
            // is_prime call expands complete set to sqrt(n) if necessary
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
    pub fn gen_divisors(&mut self, n: u64) -> Vec<u64> {
        let mut result = HashSet::new();
        result.insert(1);
        let factors = self.factorize(n);
        // TODO is there a faster way to do this?
        for factor in factors {
            if factor == 1 {
                continue;
            }
            let mut new_result = HashSet::new();
            for divisor in &result {
                let thing = divisor * factor;
                if thing <= n {
                    new_result.insert(thing);
                }
            }
            for divisor in new_result {
                result.insert(divisor);
            }
        }
        result.into_iter().collect()
    }
}

pub fn compute_collatz_length(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n == 1 {
        return 1;
    }
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    let result = if n % 2 == 0 {
        1 + compute_collatz_length(n / 2, memo)
    } else {
        1 + compute_collatz_length(3 * n + 1, memo)
    };
    memo.insert(n, result);
    result
}

pub fn _add_digits(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
    let mut result = Vec::new();
    let mut carry = 0;
    let mut a_iter = a.iter().rev();
    let mut b_iter = b.iter().rev();
    loop {
        let a_digit = a_iter.next().unwrap_or(&0);
        let b_digit = b_iter.next().unwrap_or(&0);
        let sum = a_digit + b_digit + carry;
        if sum == 0 {
            break;
        }
        result.push(sum % 10);
        carry = sum / 10;
    }
    result.reverse();
    result
}

pub fn num_to_word(n: u32) -> String {
    if n > 1000 {
        panic!("num_to_word only supports numbers up to 1000");
    }
    let mut result = String::new();
    let ones = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = vec![
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
        "eighteen", "nineteen",
    ];
    let tens = vec![
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    let hundreds = "hundred";
    let thousands = "thousand";
    let and = "and";
    if n == 1000 {
        result.push_str("one");
        result.push_str(thousands);
        return result;
    }
    if n >= 100 {
        result.push_str(ones[(n / 100 - 1) as usize]);
        result.push_str(hundreds);
        if n % 100 != 0 {
            result.push_str(and);
        }
    }
    if n % 100 >= 20 {
        result.push_str(tens[(n % 100 / 10 - 2) as usize]);
        if n % 10 != 0 {
            result.push_str(ones[(n % 10 - 1) as usize]);
        }
    } else if n % 100 >= 10 {
        result.push_str(teens[(n % 100 - 10) as usize]);
    } else if n % 100 != 0 {
        result.push_str(ones[(n % 10 - 1) as usize]);
    }
    result
}

pub struct TreeNode {
    value: u64,
    id: u64,
    left: Option<Rc<TreeNode>>,
    right: Option<Rc<TreeNode>>,
}

pub fn parse_triangle_into_vec_tree(triangle: &str) -> Vec<Vec<u64>> {
    let mut result = Vec::new();
    for line in triangle.lines() {
        let mut row = Vec::new();
        for num in line.split_whitespace() {
            row.push(num.parse().unwrap());
        }
        result.push(row);
    }
    result
}

pub fn vec_tree_into_tree_tree(vec_tree: Vec<Vec<u64>>) -> Rc<TreeNode> {
    let mut row_below = None;
    for (ix, row) in vec_tree.iter().enumerate().rev() {
        let mut current_row = Vec::new();
        for i in 0..row.len() {
            let node = Rc::new(TreeNode {
                value: row[i],
                id: (ix * 1000 + i) as u64,
                left: row_below.as_ref().map(|row_below: &Vec<Rc<TreeNode>>| Rc::clone(&row_below[i])),
                right: row_below.as_ref().map(|row_below: &Vec<Rc<TreeNode>>| Rc::clone(&row_below[i + 1])),
            });
            current_row.push(node);
        }
        row_below = Some(current_row);
    };
    row_below.unwrap()[0].clone()
}

fn max_path_sum_helper(tn: &Rc<TreeNode>, memo: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&result) = memo.get(&tn.id) {
        return result;
    }
    let left = tn.left.as_ref().map(|i| max_path_sum_helper(i,memo)).unwrap_or(0);
    let right = tn.right.as_ref().map(|i| max_path_sum_helper(i,memo)).unwrap_or(0);
    let resi = tn.value + max(left, right);
    memo.insert(tn.id, resi);
    resi
}

pub fn max_path_sum(tree: &Rc<TreeNode>) -> u64 {
    let mut memo = HashMap::new();
    max_path_sum_helper(tree, &mut memo)
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
    #[test]
    fn test_gen_divisors() {
        let mut prime_memo = super::PrimeMemo::new();
        let mut a = prime_memo.gen_divisors(1);
        a.sort();
        assert_eq!(a, vec![1]);
        let mut a = prime_memo.gen_divisors(2);
        a.sort();
        assert_eq!(a, vec![1, 2]);
        let mut a = prime_memo.gen_divisors(3);
        a.sort();
        assert_eq!(a, vec![1, 3]);
        let mut a = prime_memo.gen_divisors(4);
        a.sort();
        assert_eq!(a, vec![1, 2, 4]);
        let mut a = prime_memo.gen_divisors(6);
        a.sort();
        assert_eq!(a, vec![1, 2, 3, 6]);
        let mut a = prime_memo.gen_divisors(10);
        a.sort();
        assert_eq!(a, vec![1, 2, 5, 10]);
        let mut a = prime_memo.gen_divisors(15);
        a.sort();
        assert_eq!(a, vec![1, 3, 5, 15]);
        let mut a = prime_memo.gen_divisors(21);
        a.sort();
        assert_eq!(a, vec![1, 3, 7, 21]);
        let mut a = prime_memo.gen_divisors(28);
        a.sort();
        assert_eq!(a, vec![1, 2, 4, 7, 14, 28]);
    }
}
