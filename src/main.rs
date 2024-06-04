use rayon::prelude::*;
use std::{collections::HashMap, time::Instant};

const MIN_NUMBER: u32 = 0;
const MAX_NUMBER: u32 = 100_000_000;

fn num_digits(number: u32) -> usize {
    (number as f64).log10().floor() as usize + 1
}

fn num_morse_code_symbols(number: u32) -> usize {
    num_digits(number) * 5
}

fn length_as_string(number: u32, lengths: &HashMap<char, usize>) -> (usize, String) {
    let words = number_to_words::number_to_words(number, false);
    // dbg!(&words);
    let len = words
        .chars()
        .map(|letter| lengths.get(&letter).copied().unwrap_or(0))
        .sum();
    (len, words)
}

fn main() {
    // Number of morse boops for each letter
    let letter_lengths = [
        ('a', 2),
        ('b', 4),
        ('c', 4),
        ('d', 3),
        ('e', 1),
        ('f', 4),
        ('g', 4),
        ('h', 4),
        ('i', 2),
        ('j', 4),
        ('k', 3),
        ('l', 4),
        ('m', 2),
        ('n', 2),
        ('o', 3),
        ('p', 4),
        ('q', 4),
        ('r', 3),
        ('s', 3),
        ('t', 1),
        ('u', 3),
        ('v', 4),
        ('w', 3),
        ('x', 4),
        ('y', 4),
        ('z', 4),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let start = Instant::now();

    (MIN_NUMBER..MAX_NUMBER + 1)
        .into_par_iter()
        .for_each(|number| {
            let symbols = num_morse_code_symbols(number);
            let (string, words) = length_as_string(number, &letter_lengths);

            if symbols > string {
                println!("number: {number} symbols: {symbols}, string: {string}, {words}");
            }
        });

    let elapsed = start.elapsed();
    println!("Completed in {} seconds", elapsed.as_secs());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(0), 1);
        assert_eq!(num_digits(1), 1);
        assert_eq!(num_digits(9), 1);
        assert_eq!(num_digits(10), 2);
        assert_eq!(num_digits(99), 2);
        assert_eq!(num_digits(100), 3);
    }
}
