// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

mod parsers;
mod types;

#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::{BufRead, BufReader};

use types::Grid;

fn compute_result_task1(grid: &Grid) -> usize {
    grid.count(2)
}

fn compute_result_task2(grid: &Grid) -> i32 {
    let v = grid.intact();
    assert_eq!(v.len(), 1);
    *v.iter().next().unwrap()
}

fn main() {
    let f = File::open("input/data.txt").unwrap();
    let file = BufReader::new(&f);
    let mut grid = Grid::new();

    for line in file.lines() {
        grid.update(&parsers::parse_claim(&line.unwrap()).unwrap());
    }

    println!("Result of task 1: {}", compute_result_task1(&grid));
    println!("Result of task 2: {}", compute_result_task2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_results_task1() {
        // Sample claims.
        let mut claims = Vec::new();

        claims.push(parsers::parse_claim("#1 @ 1,3: 4x4").unwrap());
        claims.push(parsers::parse_claim("#2 @ 3,1: 4x4").unwrap());
        claims.push(parsers::parse_claim("#3 @ 5,5: 2x2").unwrap());

        // Our grid.
        let mut grid = Grid::new();

        for claim in claims {
            grid.update(&claim);
        }

        assert_eq!(grid.count(2), 4);
    }

    #[test]
    fn known_results_task2() {
        // Sample claims.
        let mut claims = Vec::new();

        claims.push(parsers::parse_claim("#1 @ 1,3: 4x4").unwrap());
        claims.push(parsers::parse_claim("#2 @ 3,1: 4x4").unwrap());
        claims.push(parsers::parse_claim("#3 @ 5,5: 2x2").unwrap());

        // Our grid.
        let mut grid = Grid::new();

        for claim in claims {
            grid.update(&claim);
        }

        let v = grid.intact();

        assert_eq!(v.len(), 1);
        assert_eq!(*v.iter().next().unwrap(), 3);
    }

    #[test]
    fn parse_claim_known_values() {
        let a = parsers::parse_claim("#1 @ 1,3: 4x4").unwrap();
        assert_eq!(a.id(), 1);
        let a_points = a.points();
        assert_eq!(a_points.len(), 16);

        let b = parsers::parse_claim("#2 @ 3,1: 4x4").unwrap();
        assert_eq!(b.id(), 2);
        let b_points = b.points();
        assert_eq!(b_points.len(), 16);

        let c = parsers::parse_claim("#3 @ 5,5: 2x2").unwrap();
        assert_eq!(c.id(), 3);
        let c_points = c.points();
        assert_eq!(c_points.len(), 4);
    }
}
