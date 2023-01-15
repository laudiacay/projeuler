use crate::util;
use std::collections::HashMap;
use num_bigint::BigUint;
use crate::util::{max_path_sum};

pub fn summation_of_primes() {
    let mut prime_memo = util::PrimeMemo::new();
    prime_memo.expand_complete_set_through(2_000_000);
    let sum = prime_memo
        .ordered_complete_prime_set
        .iter()
        .filter(|x| **x < 2_000_000)
        .sum::<u64>();
    println!("{sum}");
}

pub fn largest_product_in_a_grid() {
    let grid: Vec<Vec<u64>> = include_str!("../../data/p11.txt")
        .split('\n')
        .take(20)
        .map(|line| {
            line.split(' ')
                .take(20)
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let mut max_product = 0;
    // horizontal
    #[allow(clippy::needless_range_loop)]
    for row in 0..grid.len() {
        for col in 0..grid[row].len() - 3 {
            let product =
                grid[row][col] * grid[row][col + 1] * grid[row][col + 2] * grid[row][col + 3];
            if product > max_product {
                max_product = product;
            }
        }
    }
    // vertical
    for row in 0..grid.len() - 3 {
        for col in 0..grid[row].len() {
            let product =
                grid[row][col] * grid[row + 1][col] * grid[row + 2][col] * grid[row + 3][col];
            if product > max_product {
                max_product = product;
            }
        }
    }
    // diagonals
    for row in 0..grid.len() - 3 {
        for col in 0..grid[row].len() - 3 {
            let product = grid[row][col]
                * grid[row + 1][col + 1]
                * grid[row + 2][col + 2]
                * grid[row + 3][col + 3];
            if product > max_product {
                max_product = product;
            }
            let other_product = grid[row][col + 3]
                * grid[row + 1][col + 2]
                * grid[row + 2][col + 1]
                * grid[row + 3][col];
            if other_product > max_product {
                max_product = other_product;
            }
        }
    }
    println!("{max_product}");
}

pub fn highly_divisible_triangular_number() {
    let mut triangle = 0;
    let mut i = 1;
    let mut max_seen = 0;
    let mut prime_memo = util::PrimeMemo::new();
    loop {
        triangle += i;
        let divisors = prime_memo.gen_divisors(triangle);
        if divisors.len() > max_seen {
            max_seen = divisors.len();
            println!("{triangle} has {max_seen} divisors, last added i = {i}");
        }
        if divisors.len() >= 500 {
            println!("{triangle}");
            return;
        }
        i += 1;
    }
}

pub fn large_sum() {
    // read p13.txt
    let numbers: Vec<Vec<u32>> = include_str!("../../data/p13.txt")
        .split('\n')
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .take(100)
        .collect::<Vec<Vec<u32>>>();
    let mut sum_chain = vec![];
    for i in (0..numbers[0].len()).rev() {
        let mut sum = 0;
        #[allow(clippy::needless_range_loop)]
        for j in 0..numbers.len() {
            sum += numbers[j][i];
        }
        sum_chain.push(sum);
    }
    //let sum_chain = sum_chain.iter().rev().collect::<Vec<&u32>>();
    let mut real_sum_chain = vec![];
    let mut carry = 0;
    for num in sum_chain {
        let real_sum = num + carry;
        carry = real_sum / 10;
        let digit = real_sum % 10;
        real_sum_chain.push(digit);
    }
    while carry > 0 {
        real_sum_chain.push(carry % 10);
        carry /= 10;
    }
    let answer = real_sum_chain
        .iter()
        .rev()
        .take(10)
        .map(|x| x.to_string())
        .collect::<String>();
    println!("{answer}");
}

pub fn longest_collatz_sequence() {
    let mut max_seen = 0;
    let mut max_seen_i = 0;
    let mut memo = HashMap::new();
    for i in 1..1_000_000 {
        let collatz = util::compute_collatz_length(i, &mut memo);
        if collatz > max_seen {
            max_seen = collatz;
            max_seen_i = i;
        }
    }
    println!("{max_seen_i}");
}

fn lattice_paths_helper(x: u64, y: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if x == 0 || y == 0 {
        return 1;
    }
    if (x, y) == (1, 1) {
        return 2;
    }
    if memo.contains_key(&(x, y)) {
        return memo[&(x, y)];
    }
    let val = lattice_paths_helper(x - 1, y, memo) + lattice_paths_helper(x, y - 1, memo);
    memo.insert((x, y), val);
    val
}

pub fn lattice_paths() {
    let res = lattice_paths_helper(20, 20, &mut HashMap::new());
    println!("{res}");
}

pub fn power_digit_sum() {
    let mut num = BigUint::from(2u32).pow(1000);
    let mut sum = BigUint::from(0u32);
    while num > 0u32.into() {
        sum += num.clone() % 10u32;
        num /= 10u32;
    }
    println!("{sum}");
}

pub fn number_letter_counts() {
    let mut sum = 0;
    for i in 1..1001 {
        println!("{i}");
        println!("{}", util::num_to_word(i));
        sum += util::num_to_word(i).len();
    }
    println!("{sum}");
}

pub fn maximum_path_sum_1() {
    let gridd = util::parse_triangle_into_vec_tree(include_str!("../../data/p18.txt"));
    let vecs = util::vec_tree_into_tree_tree(gridd);
    let res = max_path_sum(&vecs);
    println!("{res}");
}
