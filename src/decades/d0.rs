use crate::util;

pub fn sum_of_multiples_of_three_or_five() {
    // If we list all the natural numbers below 10 that are multiples of 3 or 5,
    // we get 3, 5, 6 and 9. The sum of these multiples is 23.
    //
    // Find the sum of all the multiples of 3 or 5 below 1000.
    let mut sum = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("{sum}");
}

pub fn even_fibonacci_numbers() {
    // Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
    //
    // 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
    //
    // By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
    let mut memo = vec![1, 2];
    let mut sum = 2;
    loop {
        let next = util::fibonacci_memo(memo.len() as u32, &mut memo);
        if next > 4_000_000 {
            break;
        }
        if next % 2 == 0 {
            sum += next;
        }
    }
    println!("{sum}");
}

pub fn largest_prime_factor() {
    // The prime factors of 13195 are 5, 7, 13 and 29.
    //
    // What is the largest prime factor of the number 600851475143 ?
    let n = 600_851_475_143;
    let mut prime_memo = util::PrimeMemo::new();
    let factors = prime_memo.factorize(n);
    println!("{}", factors.last().unwrap());
}

pub fn largest_palindrome_product() {
    // A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
    //
    // Find the largest palindrome made from the product of two 3-digit numbers.
    let mut max = 0;
    for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            let product = i * j;
            if util::is_palindrome(product) && product > max {
                max = product;
            }
        }
    }
    println!("{max}");
}

pub fn smallest_multiple() {
    // 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
    //
    // What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
    // the numbers it needs to be divisible by are the minimal set that covers each of the prime factors from 1 to 20
    // ok, let's construct this set

    // 1
    // 2
    // 3
    // 2 2
    // 5
    // 2 3
    // 7
    // 2 2 2
    // 3 3
    // 2 5
    // 11
    // 2 2 3
    // 13
    // 2 7
    // 3 5
    // 2 2 2 2
    // 17
    // 2 3 3
    // 19
    // 2 2 5

    // 1 is covered
    // add 2 {2}
    // add 3 {2, 3}
    // add 4 {2, 2, 3}
    // add 5 {2, 2, 3, 5}
    // 6 is covered
    // add 7 {2, 2, 3, 5, 7}
    // add 8 {2, 2, 2, 3, 5, 7}
    // add 9 {2, 2, 2, 3, 3, 5, 7}
    // 10 is covered
    // add 11 {2, 2, 2, 3, 3, 5, 7, 11}
    // 12 is covered
    // add 13 {2, 2, 2, 3, 3, 5, 7, 11, 13}
    // 14 is covered
    // 15 is covered
    // add 16 {2, 2, 2, 2, 3, 3, 5, 7, 11, 13}
    // add 17 {2, 2, 2, 2, 3, 3, 5, 7, 11, 13, 17}
    // 18 is covered
    // add 19 {2, 2, 2, 2, 3, 3, 5, 7, 11, 13, 17, 19}
    // 20 is covered

    let product = 2 * 2 * 2 * 2 * 3 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
    println!("{product}");
}

pub fn sum_square_difference() {
    // The sum of the squares of the first ten natural numbers is,
    //
    // The square of the sum of the first ten natural numbers is,
    //
    // Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is .
    //
    // Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

    let sum_of_squares = (1..101).map(|x| x * x).sum::<u32>();
    let square_of_sum = (1..101).sum::<u32>().pow(2);
    println!("{}", square_of_sum - sum_of_squares);
}

pub fn nth_prime() {
    let mut prime_memo = util::PrimeMemo::new();
    let answer = prime_memo.nth_prime(10_001);
    println!("{answer}");
}

pub fn largest_product_in_a_series() {
    // the largest one is gonna be 13 digits that don't have any zeros in them
    // read p8.txt
    let window_size = 13;
    let digits: Vec<u64> = include_str!("../../data/p8.txt")
        .replace('\n', "")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|x| x as u64)
        .collect::<Vec<u64>>();
    let mut max = 0;
    // this is so slow lmfao but im not gonna prematurely optimize
    for window_start in 0..digits.len() - window_size {
        let window = &digits[window_start..window_start + 13];
        let product = window.iter().product();
        if product > max {
            max = product;
        }
    }
    println!("{max}");
}

pub fn special_pythagorean_triplet() {
    for a in 1..1000 {
        for b in a..1000 {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                println!("{}", a * b * c);
                return;
            }
        }
    }
}
