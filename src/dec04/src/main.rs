// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

mod parsers;
mod types;

#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::{BufRead, BufReader};

use types::EventTracker;

fn compute_result_task1(tracker: &EventTracker) -> usize {
    let summaries = tracker.summaries();
    let mut guard_id = 0;
    let mut minutes_asleep = 0;
    let mut minute = 0;

    for summary in summaries {
        if summary.minutes_asleep() > minutes_asleep {
            guard_id = summary.id();
            minute = summary.most_missed_timestamp().minutes();
            minutes_asleep = summary.minutes_asleep();
        }
    }

    guard_id as usize * minute as usize
}

fn compute_result_task2(tracker: &EventTracker) -> usize {
    let summaries = tracker.summaries();
    let mut guard_id = 0;
    let mut max_count = 0;
    let mut minute = 0;

    for summary in summaries {
        if summary.most_missed_timestamp_count() > max_count {
            guard_id = summary.id();
            minute = summary.most_missed_timestamp().minutes();
            max_count = summary.most_missed_timestamp_count();
        }
    }

    guard_id as usize * minute as usize
}

fn main() {
    let f = File::open("input/data.txt").unwrap();
    let file = BufReader::new(&f);
    let mut tracker = EventTracker::new();
    let mut events = Vec::new();

    for line in file.lines() {
        events.push(parsers::parse(&line.unwrap()).unwrap());
    }

    events.sort();

    for event in events.iter() {
        tracker.event(&event);
    }

    println!("Result of task 1: {}", compute_result_task1(&tracker));
    println!("Result of task 2: {}", compute_result_task2(&tracker));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_tracker() -> EventTracker {
        let lines = vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ];

        let mut tracker = EventTracker::new();

        for line in lines.iter() {
            tracker.event(&parsers::parse(&line).unwrap());
        }

        tracker
    }

    #[test]
    fn known_results_task1() {
        let tracker = build_test_tracker();
        assert_eq!(10 * 24, compute_result_task1(&tracker));
    }

    #[test]
    fn known_results_task2() {
        let tracker = build_test_tracker();
        assert_eq!(99 * 45, compute_result_task2(&tracker));
    }
}
