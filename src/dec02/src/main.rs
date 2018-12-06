// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn compute_result_task1(lines: &Vec<String>) -> i32 {
    let mut two = 0;
    let mut three = 0;
    let mut letter_frequency = HashMap::new();

    for line in lines {
        // Build a table containing the frequency of each letter.
        for c in line.chars() {
            *letter_frequency.entry(c).or_insert(0) += 1
        }

        // Increment our `two` count by one iff any letter appears twice.
        if letter_frequency.values().any(|&f| f == 2) {
            two += 1
        }

        // Increment our `three` count by one iff any letter appears thrice.
        if letter_frequency.values().any(|&f| f == 3) {
            three += 1
        }

        letter_frequency.clear();
    }

    two * three
}

fn check_strings(a: &String, b: &String) -> Option<String> {
    assert_eq!(a.len(), b.len());

    // Build a sequence of pairs with the letter of each string.
    let letters = a.chars().zip(b.chars());

    // Build a new string where letters at the same position of `a` and `b` are removed.
    let result: String = letters
        .filter_map(|(v_a, v_b)| if v_a == v_b { Some(v_a) } else { None })
        .collect();

    // If our stripped string in `result` have one less character than the input, we have the
    // result.
    if a.len() - 1 == result.len() {
        return Some(result);
    }

    // This was not the result we were looking for.
    None
}

fn compute_result_task2_naive(lines: &Vec<String>) -> String {
    // Naive O(n^2) algorithm.
    for (index, a) in lines.iter().enumerate() {
        for b in lines.iter().skip(index + 1) {
            if let Some(result) = check_strings(a, b) {
                return result;
            }
        }
    }

    String::from("Not found")
}

fn compute_result_task2_linear(lines: &Vec<String>) -> String {
    // Linear function. Not benchmarked yet.
    let mut set = HashSet::new();

    for line in lines {
        // We use '@' as replacement character. Make sure our input string does not contain any
        // '@' characters.
        assert!(!line.contains("@"));

        // For each character in the line, create a new string where a character is replaced by
        // '@'.
        //
        // For example:
        //   "foobar" -> ["@oobar", "f@obar", "fo@bar", "foo@ar", "foob@r", "fooba@"].
        for (index, _) in line.chars().enumerate() {
            let new_line: String = line
                .chars()
                .enumerate()
                .map(|(i, c)| if i == index { '@' } else { c })
                .collect();

            assert_eq!(new_line.len(), line.len());

            // If our mutated string already exists, remove all '@' from the string and return the
            // result.
            if !set.insert(new_line.clone()) {
                return new_line
                    .chars()
                    .filter_map(|c| if c != '@' { Some(c) } else { None })
                    .collect();
            }
        }
    }

    String::from("Not found")
}

fn main() {
    let f = File::open("input/data.txt").unwrap();
    let file = BufReader::new(&f);
    let mut lines = Vec::new();

    for line in file.lines() {
        lines.push(line.unwrap());
    }

    println!("Result of task 1: {}", compute_result_task1(&lines));
    println!(
        "Result of task 2: {} (linear)",
        compute_result_task2_linear(&lines)
    );
    println!(
        "Result of task 2: {} (naive)",
        compute_result_task2_naive(&lines)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_results_task1() {
        let mut lines = Vec::new();
        lines.push(String::from("abcdef"));
        lines.push(String::from("bababc"));
        lines.push(String::from("abbcde"));
        lines.push(String::from("abcccd"));
        lines.push(String::from("aabcdd"));
        lines.push(String::from("abcdee"));
        lines.push(String::from("ababab"));

        assert_eq!(12, compute_result_task1(&lines));
    }

    #[test]
    fn known_results_task2() {
        let mut lines = Vec::new();
        lines.push(String::from("abcde"));
        lines.push(String::from("fghij"));
        lines.push(String::from("klmno"));
        lines.push(String::from("pqrst"));
        lines.push(String::from("fguij"));
        lines.push(String::from("axcye"));
        lines.push(String::from("wvxyz"));

        assert_eq!("fgij", compute_result_task2_naive(&lines));
        assert_eq!("fgij", compute_result_task2_linear(&lines));
    }
}
