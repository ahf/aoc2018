// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::cmp::Ordering;
use std::collections::HashMap;

pub type GuardID = u32;

#[derive(Debug, Eq, PartialEq)]
pub enum EventType {
    // Our guard wakes up.
    GuardAwake,

    // Our guard falls asleep.
    GuardAsleep,

    // New guard begins their shift.
    GuardBeginsShift(GuardID),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Event {
    // Our date + time.
    datetime: DateTime,

    // Our event type.
    event_type: EventType,
}

impl Event {
    pub fn new(datetime: DateTime, event_type: EventType) -> Event {
        Event {
            datetime,
            event_type,
        }
    }

    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }

    pub fn datetime(&self) -> &DateTime {
        &self.datetime
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        self.datetime.cmp(&other.datetime)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Date {
    // Our year.
    year: i32,

    // Our month.
    month: u32,

    // Our day.
    day: u32,
}

impl Date {
    pub fn new(year: i32, month: u32, day: u32) -> Date {
        Date { year, month, day }
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn day(&self) -> u32 {
        self.day
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Time {
    // Our hour.
    hour: u8,

    // Our minutes.
    minutes: u8,
}

impl Time {
    pub fn new(hour: u8, minutes: u8) -> Time {
        Time { hour, minutes }
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minutes(&self) -> u8 {
        self.minutes
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DateTime {
    // Our date.
    date: Date,

    // Our time.
    time: Time,
}

impl DateTime {
    pub fn new(date: Date, time: Time) -> DateTime {
        DateTime { date, time }
    }

    pub fn date(&self) -> &Date {
        &self.date
    }

    pub fn time(&self) -> &Time {
        &self.time
    }
}

pub struct GuardSummary {
    // The ID of our guard.
    id: GuardID,

    // Time slot where the guard was most often asleep.
    most_missed_timestamp: Time,

    // Amount of times the guard was asleep at `most_missed_timestamp`.
    most_missed_timestamp_count: usize,

    // Total minutes slept.
    minutes_asleep: usize,
}

impl GuardSummary {
    pub fn new(
        id: GuardID,
        most_missed_timestamp: Time,
        most_missed_timestamp_count: usize,
        minutes_asleep: usize,
    ) -> GuardSummary {
        GuardSummary {
            id,
            most_missed_timestamp,
            most_missed_timestamp_count,
            minutes_asleep,
        }
    }

    pub fn id(&self) -> GuardID {
        self.id
    }

    pub fn most_missed_timestamp(&self) -> &Time {
        &self.most_missed_timestamp
    }

    pub fn most_missed_timestamp_count(&self) -> usize {
        self.most_missed_timestamp_count
    }

    pub fn minutes_asleep(&self) -> usize {
        self.minutes_asleep
    }
}

pub struct EventTracker {
    // The current guard on duty.
    current_guard: GuardID,

    // Tracking of minutes asleep for each guard.
    sleep_tracker: HashMap<GuardID, SleepTracker>,
}

impl EventTracker {
    pub fn new() -> EventTracker {
        EventTracker {
            current_guard: 0,
            sleep_tracker: HashMap::new(),
        }
    }

    pub fn event(&mut self, event: &Event) {
        match event.event_type() {
            EventType::GuardAsleep => self.asleep(event.datetime()),
            EventType::GuardAwake => self.awake(event.datetime()),
            EventType::GuardBeginsShift(guard) => self.begins_shift(guard),
        }
    }

    fn asleep(&mut self, datetime: &DateTime) {
        self.sleep_tracker
            .get_mut(&self.current_guard)
            .unwrap()
            .asleep(datetime);
    }

    fn awake(&mut self, datetime: &DateTime) {
        self.sleep_tracker
            .get_mut(&self.current_guard)
            .unwrap()
            .awake(datetime);
    }

    fn begins_shift(&mut self, guard: &GuardID) {
        if !self.sleep_tracker.contains_key(guard) {
            self.sleep_tracker.insert(*guard, SleepTracker::new());
        }

        self.current_guard = *guard;
    }

    pub fn summaries(&self) -> Vec<GuardSummary> {
        // Our result.
        let mut result = Vec::new();

        for (guard_id, sleep_tracker) in self.sleep_tracker.iter() {
            // Mapping between `(Hour, Minute) -> Count` used to keep track at which time the given
            // Guard sleeps the most.
            let mut asleep_time_freq = HashMap::new();

            // The sum of how many hours the guard is asleep at.
            let mut total_minutes_asleep = 0;

            // Loop over each minute the given guard was asleep and sum up the count.
            for minute in sleep_tracker.minutes_asleep().iter() {
                // Bump the value by 1.
                *asleep_time_freq.entry(minute.clone()).or_insert(0) += 1;

                // Bump our sum.
                total_minutes_asleep += 1;
            }

            // Figure out which point in time the guard is mostly asleep.
            let mut most_missed_timestamp_count = 0;
            let mut most_missed_timestamp = (0, 0);

            for (timestamp, count) in asleep_time_freq.iter() {
                if count > &most_missed_timestamp_count {
                    most_missed_timestamp = timestamp.clone();
                    most_missed_timestamp_count = count.clone();
                }
            }

            result.push(GuardSummary::new(
                *guard_id,
                Time::new(most_missed_timestamp.0 as u8, most_missed_timestamp.1 as u8),
                most_missed_timestamp_count,
                total_minutes_asleep,
            ));
        }

        result
    }
}

struct SleepDuration {
    start: DateTime,
    duration: u32,
}

impl SleepDuration {
    pub fn new(start: DateTime, duration: u32) -> SleepDuration {
        SleepDuration { start, duration }
    }

    pub fn minutes_asleep(&self) -> Vec<(u32, u32)> {
        let time = self.start.time();
        let mut v = Vec::new();

        for minute in 0..self.duration {
            let mut hour = time.hour() as u32;
            let mut min = time.minutes() as u32 + minute;

            if min > 59 {
                hour += 1;
                min = 0;
            }

            v.push((hour, min));
        }

        v
    }
}

enum SleepTrackerState {
    Invalid,
    Asleep(DateTime),
    Awake,
}

pub struct SleepTracker {
    // Current state.
    state: SleepTrackerState,

    // Periods where we are asleep.
    sleep_periods: Vec<SleepDuration>,
}

impl SleepTracker {
    pub fn new() -> SleepTracker {
        SleepTracker {
            state: SleepTrackerState::Invalid,
            sleep_periods: Vec::new(),
        }
    }

    // We fell asleep.
    pub fn asleep(&mut self, datetime: &DateTime) {
        self.state = SleepTrackerState::Asleep(datetime.clone());
    }

    // We woke up.
    pub fn awake(&mut self, other: &DateTime) {
        if let SleepTrackerState::Asleep(ref datetime) = self.state {
            // They always work on one day at a time.
            assert_eq!(datetime.date().year(), other.date().year());
            assert_eq!(datetime.date().month(), other.date().month());
            assert_eq!(datetime.date().day(), other.date().day());

            let delta_minutes = (other.time().hour() - datetime.time().hour()) * 60
                + (other.time().minutes() - datetime.time().minutes());

            self.sleep_periods
                .push(SleepDuration::new(datetime.clone(), delta_minutes.into()));
        } else {
            // We arrived from a state which was not `Asleep`? Sounds weird.
            assert!(false);
        }

        self.state = SleepTrackerState::Awake;
    }

    // Returns a vector of all the minutes we have been asleep at.
    pub fn minutes_asleep(&self) -> Vec<(u32, u32)> {
        let mut m = Vec::new();

        for x in self.sleep_periods.iter() {
            m.extend(x.minutes_asleep());
        }

        m
    }
}
