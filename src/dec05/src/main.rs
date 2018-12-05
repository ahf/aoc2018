// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::cmp::min;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

// Returns the reversed case of the given character. If the input character is lowercased we return
// the uppercased version and if the input character is uppercased we return the lowercased
// version.
fn reverse_case(c: char) -> char {
    assert!(c.is_ascii());

    if c.is_ascii_lowercase() {
        c.to_ascii_uppercase()
    } else {
        c.to_ascii_lowercase()
    }
}

// O(n) algorithm that returns a set of lower cased characters that is used in the given input
// string.
fn unique_characters(input: &str) -> HashSet<char> {
    let mut result = HashSet::new();

    for character in input.chars() {
        assert!(character.is_ascii());

        result.insert(character.to_ascii_lowercase());
    }

    result
}

fn reduce_with_filter(input: &str, filter: Option<char>) -> String {
    // Our resulting string. We allocate with a capacity equal to the size of the input string even
    // though our result is going to be smaller, but because of this we can avoid having to grow
    // the internal buffer when pushing characters onto the result.
    let mut result = String::with_capacity(input.len());

    for character in input.chars() {
        assert!(character.is_ascii());

        // If we have a filter, check if our character matches the character to filter. If it does
        // we continue processing our string without adding our character to the result.
        if let Some(filter_character) = filter {
            if character.eq_ignore_ascii_case(&filter_character) {
                continue
            }
        }

        // If we have had a character before (this is only false for the first character in the
        // input string) we check if our current character is equal to the previous character but
        // where their ASCII casing is reversed. If that is the case, we remove the last character
        // from our result string and continue processing our list of characters with the current
        // character discarded.
        if let Some(last_character) = result.chars().last() {
            if last_character == reverse_case(character) {
                result.pop();
                continue;
            }
        }

        result.push(character);
    }

    result
}

fn reduce(input: &str) -> String {
    reduce_with_filter(input, None)
}

fn compute_result_task1(input: &str) -> usize {
    reduce(input).len()
}

fn compute_result_task2(input: &str) -> usize {
    let characters = unique_characters(input);
    let mut result = std::usize::MAX;

    for character in characters {
        result = min(result, reduce_with_filter(input, Some(character)).len());
    }

    result
}

fn main() {
    let mut file = File::open("input/data.txt").expect("Unable to open input file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read input file");

    // Remove trailing newline.
    content.pop();

    println!("Result of task 1: {}", compute_result_task1(&content));
    println!("Result of task 2: {}", compute_result_task2(&content));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_case_test() {
        assert_eq!(reverse_case('A'), 'a');
        assert_eq!(reverse_case('a'), 'A');
        assert_eq!(reverse_case('B'), 'b');
        assert_eq!(reverse_case('b'), 'B');
        assert_eq!(reverse_case('@'), '@');
    }

    #[test]
    fn unique_characters_test() {
        let h = unique_characters("AAAaaaBBBcCcbbb@@@");

        assert_eq!(h.len(), 4);
        assert!(h.contains(&'a'));
        assert!(h.contains(&'b'));
        assert!(h.contains(&'c'));
        assert!(h.contains(&'@'));
    }

    #[test]
    fn known_results_task1() {
        assert_eq!(reduce("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
        assert_eq!(compute_result_task1("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn known_results_task2() {
        assert_eq!(reduce_with_filter("dabAcCaCBAcCcaDA", Some('a')), "dbCBcD");
        assert_eq!(reduce_with_filter("dabAcCaCBAcCcaDA", Some('b')), "daCAcaDA");
        assert_eq!(reduce_with_filter("dabAcCaCBAcCcaDA", Some('c')), "daDA");
        assert_eq!(reduce_with_filter("dabAcCaCBAcCcaDA", Some('d')), "abCBAc");

        assert_eq!(compute_result_task2("dabAcCaCBAcCcaDA"), 4);
    }
}
