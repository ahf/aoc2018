// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use nom;
use nom::types::CompleteStr;

use types::{Date, DateTime, Event, EventType, GuardID, Time};

// Parse an integer literal into a u32.
named!(integer<CompleteStr, u32>,
    flat_map!(nom::digit, parse_to!(u32)));

// Parse a Guard ID.
named!(guard_id<CompleteStr, GuardID>,
    do_parse!(tag!("#")   >>
              id: integer >>
              (id)));

// Parse a year.
named!(year<CompleteStr, i32>,
   flat_map!(nom::digit, parse_to!(i32)));

// Parse a month.
named!(month<CompleteStr, u32>,
   flat_map!(nom::digit, parse_to!(u32)));

// Parse a day.
named!(day<CompleteStr, u32>,
   flat_map!(nom::digit, parse_to!(u32)));

// Parse a date.
named!(date<CompleteStr, Date>,
    do_parse!(year: year   >>
              tag!("-")    >>
              month: month >>
              tag!("-")    >>
              day: day     >>
              (Date::new(year, month, day))));

// Parse hours.
named!(hours<CompleteStr, u8>,
   flat_map!(nom::digit, parse_to!(u8)));

// Parse minutes.
named!(minutes<CompleteStr, u8>,
   flat_map!(nom::digit, parse_to!(u8)));

// Parse time.
named!(time<CompleteStr, Time>,
    do_parse!(hours: hours     >>
              tag!(":")        >>
              minutes: minutes >>
              (Time::new(hours, minutes))));

// Parse a date+time.
named!(datetime<CompleteStr, DateTime>,
    do_parse!(date: date >>
              tag!(" ")  >>
              time: time >>
              (DateTime::new(date, time))));

// Parse a "wakes up" event type.
named!(wakes_up<CompleteStr, EventType>,
    do_parse!(tag!("wakes up") >> (EventType::GuardAwake)));

// Parse a "falls asleep" event type.
named!(falls_asleep<CompleteStr, EventType>,
    do_parse!(tag!("falls asleep") >> (EventType::GuardAsleep)));

// Parse a "begins shift" event type.
named!(begins_shift<CompleteStr, EventType>,
    do_parse!(tag!("Guard")        >>
              id: ws!(guard_id)    >>
              tag!("begins shift") >>
              (EventType::GuardBeginsShift(id))));

// Parse an event type.
named!(event_type<CompleteStr, EventType>,
    alt!(wakes_up | falls_asleep | begins_shift));

// Parse an event.
named!(event<CompleteStr, Event>,
    do_parse!(tag!("[")                   >>
              datetime: datetime          >>
              tag!("]")                   >>
              event_type: ws!(event_type) >>
              (Event::new(datetime, event_type))));

pub fn parse(s: &str) -> Option<Event> {
    match event(CompleteStr(s)) {
        Ok((CompleteStr(""), event)) => Some(event),
        _ => None,
    }
}
