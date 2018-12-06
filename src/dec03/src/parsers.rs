// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use nom;
use nom::types::CompleteStr;

use types::{Claim, Dimension, Point};

named!(integer<CompleteStr, i32>,
    flat_map!(nom::digit, parse_to!(i32)));

named!(point<CompleteStr, Point>,
    do_parse!(x: integer >>
              tag_s!(",")  >>
              y: integer >>
              (Point::new(x, y))));

named!(dimension<CompleteStr, Dimension>,
    do_parse!(width: integer  >>
              tag!("x")       >>
              height: integer >>
              (Dimension::new(width, height))));

named!(claim<CompleteStr, Claim>,
    do_parse!(tag!("#")            >>
              id: integer          >>
              ws!(tag!("@"))       >>
              point: point         >>
              ws!(tag!(":"))       >>
              dimension: dimension >>
              (Claim::new(id, point, dimension))));

pub fn parse_claim(s: &str) -> Option<Claim> {
    match claim(CompleteStr(s)) {
        Ok((CompleteStr(""), claim)) => Some(claim),
        _ => None,
    }
}
