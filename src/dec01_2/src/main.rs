// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::collections::{HashSet, LinkedList};

use std::fs::File;
use std::io::{BufRead, BufReader};

fn compute_result(values: &LinkedList<i32>) -> i32 {
    let mut counter: i32 = 0;

    let mut state: HashSet<i32> = HashSet::new();
    state.insert(counter);

    for value in values.iter().cycle() {
        counter += *value;

        if ! state.insert(counter) {
            break
        }
    }

    counter
}

fn main() {
    let f = File::open("input/data.txt").unwrap();
    let file = BufReader::new(&f);
    let mut values = LinkedList::new();

    for line in file.lines() {
        let value = line.unwrap().parse::<i32>().unwrap();
        values.push_back(value)
    }

    println!("Result: {}", compute_result(&values))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compute_test_result(values: &[i32]) -> i32 {
        let mut v = LinkedList::new();
        v.extend(values);

        compute_result(&v)
    }

    #[test]
    fn known_results() {
        assert_eq!(0, compute_test_result(&[1, -1]));
        assert_eq!(10, compute_test_result(&[3, 3, 4, -2, -4]));
        assert_eq!(5, compute_test_result(&[-6, 3, 8, 5, -6]));
        assert_eq!(14, compute_test_result(&[7, 7, -2, -7, -4]));
    }
}
