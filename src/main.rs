#![feature(bigint_helper_methods)]
#![deny(unused_crate_dependencies)]
use clap::Parser;

mod decades;
mod util;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "problem id")]
    problem: u32,
}

fn main() {
    let args = Args::parse();
    match args.problem {
        1 => decades::zero::sum_of_multiples_of_three_or_five(),
        2 => decades::zero::even_fibonacci_numbers(),
        3 => decades::zero::largest_prime_factor(),
        4 => decades::zero::largest_palindrome_product(),
        5 => decades::zero::smallest_multiple(),
        6 => decades::zero::sum_square_difference(),
        7 => decades::zero::nth_prime(),
        8 => decades::zero::largest_product_in_a_series(),
        9 => decades::zero::special_pythagorean_triplet(),
        _ => panic!("problem {} not implemented", args.problem),
        // 10 => decades::one::summation_of_primes(),
        // 11 => decades::one::largest_product_in_a_grid(),
        // 12 => decades::one::highly_divisible_triangular_number(),
        // 13 => decades::one::large_sum(),
        // 14 => decades::one::longest_collatz_sequence(),
        // 15 => decades::one::lattice_paths(),
        // 16 => decades::one::power_digit_sum(),
        // 17 => decades::one::number_letter_counts(),
        // 18 => decades::one::maximum_path_sum(),
        // 19 => decades::one::counting_sundays(),
        // 20 => decades::two::factorial_digit_sum(),
        // 21 => decades::two::amicable_numbers(),
        // 22 => decades::two::names_scores(),
        // 23 => decades::two::non_abundant_sums(),
        // 24 => decades::two::lexicographic_permutations(),
        // 25 => decades::two::nth_fibonacci_number(),
        // 26 => decades::two::reciprocal_cycles(),
        // 27 => decades::two::quadratic_primes(),
        // 28 => decades::two::number_spiral_diagonals(),
        // 29 => decades::two::distinct_powers(),
        // 30 => decades::three::digit_fifth_powers(),
        // 31 => decades::three::coin_sums(),
        // 32 => decades::three::pandigital_products(),
    }
}
