// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::cmp::{min, max};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

#[derive(Debug)]
pub struct Dimension {
    width: i32,
    height: i32,
}

impl Dimension {
    pub fn new(width: i32, height: i32) -> Dimension {
        Dimension { width, height }
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn width(&self) -> i32 {
        self.width
    }
}

type ClaimID = i32;

#[derive(Debug)]
pub struct Claim {
    id: ClaimID,
    point: Point,
    dimension: Dimension,
}

impl Claim {
    pub fn new(id: ClaimID, point: Point, dimension: Dimension) -> Claim {
        Claim { id, point, dimension }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> ClaimID {
        self.id
    }

    pub fn points(&self) -> Vec<Point> {
        let mut v = Vec::new();

        for x in self.point.x()..(self.point.x() + self.dimension.width()) {
            for y in self.point.y()..(self.point.y() + self.dimension.height()) {
                v.push(Point::new(x, y));
            }
        }

        v
    }
}

type Level = usize;

pub struct Grid {
    // Our Grid: This is a mapping of an X and Y coordinate to the level of the given grid cell.
    // The level is defines by the number of other claims that have claimed the given cell.
    grid: HashMap<Point, Level>,

    // Our most recent visitor of a given X and Y coordinate. This map contains a mapping between a
    // Point and a Claim ID. The Claim ID value is the most recent visitor (the current "top" of
    // the given cell).
    recent_visitor: HashMap<Point, ClaimID>,

    // Our set of Claim ID's that are intact. In this context intact means that no other Claim ID
    // have claimed the same area (and thus every Point of the given Claim ID have a level of 1 in
    // the grid member.
    intact: HashSet<ClaimID>,

    // The most recently seen Claim ID.
    last_id: ClaimID,

    // The smallest X value we have seen.
    min_x: i32,

    // The largest X value we have seen.
    max_x: i32,

    // The smallest Y value we have seen.
    min_y: i32,

    // The largest Y value we have seen.
    max_y: i32,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            grid: HashMap::new(),
            recent_visitor: HashMap::new(),
            intact: HashSet::new(),
            last_id: 0,
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }

    pub fn update(&mut self, claim: &Claim) {
        // We assume that the incoming Claim have a larger ID than any previous Claim's ID's.
        assert!(self.last_id < claim.id);
        self.last_id = claim.id;

        // By default a new claim have yet to overlap with any other claims. We consider it to be
        // intact for now, but will very soon discover if this assumption is true.
        self.intact.insert(claim.id);

        for point in claim.points() {
            // Bookkeeping used for pretty printing the board in the print() method. It is worth
            // mentioning here that you should not try to print the final board that is generated
            // after data.txt have been parsed.
            self.min_x = min(self.min_x, point.x);
            self.max_x = max(self.max_x, point.x);

            self.min_y = min(self.min_y, point.y);
            self.max_y = max(self.max_y, point.y);

            // We are now the most recent visitor of the current point. If the current point have
            // already had a visitor it means we overlap with one or more former visitors. Update
            // our `intact` set with this information.
            if let Some(last_claim_id) = self.recent_visitor.insert(point.clone(), claim.id) {
                self.intact.remove(&last_claim_id);
                self.intact.remove(&claim.id);
            }

            // Bump the level by one.
            *self.grid.entry(point).or_insert(0) += 1;
        }
    }

    pub fn count(&self, min_level: usize) -> usize {
        self.grid.values().filter(|l| **l >= min_level).count()
    }

    pub fn intact(&self) -> Vec<ClaimID> {
        Vec::from_iter(self.intact.iter().cloned())
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for x in self.min_x..(self.max_x + 2) {
            for y in self.min_y..(self.max_y + 2) {
                match self.grid.get(&Point::new(x, y)) {
                    Some(l) => print!("{}", l),
                    None    => print!("."),
                }
            }

            println!("");
        }
    }
}
