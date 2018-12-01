// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::collections::LinkedList;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn compute_result(values: &LinkedList<i32>) -> i32 {
    values.iter().sum()
}

fn main() {
    let f = File::open("input/data.txt").unwrap();
    let file = BufReader::new(&f);
    let mut values = LinkedList::new();

    for line in file.lines() {
        let value = line.unwrap().parse::<i32>().unwrap();
        values.push_back(value)
    }

    println!("Result: {}", compute_result(&values));
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
        assert_eq!(3, compute_test_result(&[1, 1, 1]));
        assert_eq!(0, compute_test_result(&[1, 1, -2]));
        assert_eq!(-6, compute_test_result(&[-1, -2, -3]));
    }
}
