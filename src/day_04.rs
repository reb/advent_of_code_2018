/// --- Day 4: Repose Record ---
///
/// You've sneaked into another supply closet - this time, it's across from the
/// prototype suit manufacturing lab. You need to sneak inside and fix the
/// issues with the suit, but there's a guard stationed outside the lab, so this
/// is as close as you can safely get.
///
/// As you search the closet for anything that might help, you discover that
/// you're not the first person to want to sneak in. Covering the walls, someone
/// has spent an hour starting every midnight for the past few months secretly
/// observing this guard post! They've been writing down the ID of the one guard
/// on duty that night - the Elves seem to have decided that one guard was
/// enough for the overnight shift - as well as when they fall asleep or wake up
/// while at their post (your puzzle input).
///
/// For example, consider the following records, which have already been
/// organized into chronological order:
///
/// [1518-11-01 00:00] Guard #10 begins shift
/// [1518-11-01 00:05] falls asleep
/// [1518-11-01 00:25] wakes up
/// [1518-11-01 00:30] falls asleep
/// [1518-11-01 00:55] wakes up
/// [1518-11-01 23:58] Guard #99 begins shift
/// [1518-11-02 00:40] falls asleep
/// [1518-11-02 00:50] wakes up
/// [1518-11-03 00:05] Guard #10 begins shift
/// [1518-11-03 00:24] falls asleep
/// [1518-11-03 00:29] wakes up
/// [1518-11-04 00:02] Guard #99 begins shift
/// [1518-11-04 00:36] falls asleep
/// [1518-11-04 00:46] wakes up
/// [1518-11-05 00:03] Guard #99 begins shift
/// [1518-11-05 00:45] falls asleep
/// [1518-11-05 00:55] wakes up
///
/// Timestamps are written using year-month-day hour:minute format. The guard
/// falling asleep or waking up is always the one whose shift most recently
/// started. Because all asleep/awake times are during the midnight hour (00:00
/// - 00:59), only the minute portion (00 - 59) is relevant for those events.
///
/// Visually, these records show that the guards are asleep at these times:
///
/// Date   ID   Minute
///             000000000011111111112222222222333333333344444444445555555555
///             012345678901234567890123456789012345678901234567890123456789
/// 11-01  #10  .....####################.....#########################.....
/// 11-02  #99  ........................................##########..........
/// 11-03  #10  ........................#####...............................
/// 11-04  #99  ....................................##########..............
/// 11-05  #99  .............................................##########.....
///
/// The columns are Date, which shows the month-day portion of the relevant day;
/// ID, which shows the guard on duty that day; and Minute, which shows the
/// minutes during which the guard was asleep within the midnight hour. (The
/// Minute column's header shows the minute's ten's digit in the first row and
/// the one's digit in the second row.) Awake is shown as ., and asleep is shown
/// as #.
///
/// Note that guards count as asleep on the minute they fall asleep, and they
/// count as awake on the minute they wake up. For example, because Guard #10
/// wakes up at 00:25 on 1518-11-01, minute 25 is marked as awake.
///
/// If you can figure out the guard most likely to be asleep at a specific time,
/// you might be able to trick that guard into working tonight so you can have
/// the best chance of sneaking in. You have two strategies for choosing the
/// best guard/minute combination.
///
/// Strategy 1: Find the guard that has the most minutes asleep. What minute
/// does that guard spend asleep the most?
///
/// In the example above, Guard #10 spent the most minutes asleep, a total of 50
/// minutes (20+25+5), while Guard #99 only slept for a total of 30 minutes
/// (10+10+10). Guard #10 was asleep most during minute 24 (on two days, whereas
/// any other minute the guard was asleep was only seen on one day).
///
/// While this example listed the entries in chronological order, your entries
/// are in the order you found them. You'll need to organize them before they
/// can be analyzed.
///
/// What is the ID of the guard you chose multiplied by the minute you chose?
/// (In the above example, the answer would be 10 * 24 = 240.)

use chrono::{NaiveDateTime, Timelike};
use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_04.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Event {
    GuardStarts(u16),
    FallsAsleep,
    WakesUp,
}

pub fn run() {
    let input = get_input();

    let sleep_map = guards_sleep_map(input);

    for (guard_number, asleep) in sleep_map.iter() {
        println!("Guard: {}", guard_number);
        for minute in asleep.iter() {
            print!("{}", minute);
        }
        println!("");
    }
}

fn guards_sleep_map(input: Vec<(NaiveDateTime, Event)>) -> HashMap<u16, [u16; 60]> {
    let mut sleep_map = HashMap::new();
    let mut current_guard = None;
    let mut fell_asleep_at = None;

    for (timestamp, event) in input.iter() {
        match event {
            Event::GuardStarts(guard_number) => {
                current_guard = Some(*guard_number);
            },
            Event::FallsAsleep => {
                if current_guard.is_some() {
                    fell_asleep_at = Some(timestamp.minute());
                }
            },
            Event::WakesUp => {
                match (current_guard, fell_asleep_at) {
                    (Some(guard), Some(start)) => {
                        let stop = timestamp.minute();
                        let mut asleep = sleep_map.entry(guard)
                            .or_insert([0; 60]);
                        for i in start..stop {
                            asleep[i as usize] += 1;
                        }
                    },
                    _ => {},
                };
            },
        };
    }
    sleep_map
}

fn get_input() -> Vec<(NaiveDateTime, Event)> {
    let mut output: Vec<_> = INPUT.lines()
        .filter_map(|line| convert_line(line))
        .collect();
    output.sort();
    output
}

fn convert_line(line: &str) -> Option<(NaiveDateTime, Event)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[(.*)\] (.*)").unwrap();
    }
    let captures = RE.captures(line).unwrap();
    match (captures.get(1), captures.get(2)) {
        (Some(timestamp), Some(message)) =>
            Some((NaiveDateTime::parse_from_str(timestamp.as_str(), "%Y-%m-%d %H:%M").unwrap(),
            convert_to_event(message.as_str()))),
        _ => None,
    }
}

fn convert_to_event(message: &str) -> Event {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    }
    match message {
        "falls asleep" => Event::FallsAsleep,
        "wakes up" => Event::WakesUp,
        other => {
            let capture = RE.captures(other).unwrap();
            match capture.get(1) {
                Some(guard_number) => Event::GuardStarts(guard_number.as_str().parse().unwrap()),
                _ => panic!(),
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_convert_line() {
        let input = "[2000-12-31 03:54] falls asleep";
        let output = Some((
            NaiveDate::from_ymd(2000, 12, 31).and_hms(3, 54, 0),
            Event::FallsAsleep
        ));

        assert_eq!(convert_line(input), output);
    }

    #[test]
    fn test_convert_to_event_falls_asleep() {
        let input = "falls asleep";
        let output = Event::FallsAsleep;

        assert_eq!(convert_to_event(input), output);
    }

    #[test]
    fn test_convert_to_event_wakes_up() {
        let input = "wakes up";
        let output = Event::WakesUp;

        assert_eq!(convert_to_event(input), output);
    }

    #[test]
    fn test_convert_to_event_guard_starts() {
        let input = "Guard #10 begins shift";
        let output = Event::GuardStarts(10);

        assert_eq!(convert_to_event(input), output);
    }

    #[test]
    #[should_panic]
    fn test_convert_to_event_panic() {
        let input = "weirdstuff";
        convert_to_event(input);
    }
}
