//-----------------------------------------------------
// Setup.

use chrono::NaiveDate;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

static INPUT: &str = include_str!("data/q04.data");

#[derive(Clone, Debug)]
enum Record {
    // [1518-07-26 23:52] Guard #1597 begins shift
    Started {
        date: String,
        guard: String,
    },
    // [1518-02-09 00:56] falls asleep
    // [1518-02-09 00:57] wakes up
    Switched {
        date: String,
        minute: u32,
        awake: bool,
    },
}

#[derive(Clone, Debug)]
struct WatchDay {
    guard: String,
    times: Vec<(u32, bool)>,
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Record, ()> {
        // #1 @ 1,3: 4x4
        let started_re: &Regex =
            regex!(r"\[(\d+)-(\d+)-(\d+) (\d+):\d+\] Guard #(\d+) begins shift");
        let slept_re: &Regex = regex!(r"\[\d+-(\d+-\d+) \d+:(\d+)\] falls asleep");
        let woke_re: &Regex = regex!(r"\[\d+-(\d+-\d+) \d+:(\d+)\] wakes up");

        if let Some(cap) = started_re.captures(s) {
            let mut date = NaiveDate::from_ymd_opt(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
            )
            .unwrap();
            let hour: u32 = cap[4].parse().unwrap();
            let guard = cap[5].to_owned();

            if hour > 22 {
                date = date.succ_opt().unwrap();
            }
            return Ok(Record::Started {
                date: date.format("%m-%d").to_string(),
                guard,
            });
        }

        if let Some(cap) = slept_re.captures(s) {
            let date = cap[1].to_owned();
            let minute: u32 = cap[2].parse().unwrap();

            return Ok(Record::Switched {
                date,
                minute,
                awake: false,
            });
        }

        if let Some(cap) = woke_re.captures(s) {
            let date = cap[1].to_owned();
            let minute: u32 = cap[2].parse().unwrap();

            return Ok(Record::Switched {
                date,
                minute,
                awake: true,
            });
        }

        Err(())
    }
}

fn process_data_a(data: &str) -> usize {
    let mut days = HashMap::new();
    for line in data.lines() {
        let record: Record = line.parse().unwrap();
        match record {
            Record::Started { date, guard } => {
                let day = days.entry(date).or_insert(WatchDay {
                    guard: "".to_owned(),
                    times: vec![],
                });
                day.guard = guard.clone();
            }
            Record::Switched {
                date,
                minute,
                awake,
            } => {
                let day = days.entry(date).or_insert(WatchDay {
                    guard: "".to_owned(),
                    times: vec![],
                });
                day.times.push((minute, awake));
                day.times.sort_unstable();
            }
        }
    }

    let awake = [0; 60];
    let asleep = [1; 60];
    let mut guards = HashMap::new();
    let mut day_sleeps = HashMap::new();
    for (date, day) in &days {
        let guard = guards.entry(&day.guard).or_insert(0);
        let sleeps = day_sleeps.entry(date).or_insert([0; 60]);
        for time in &day.times {
            let index = time.0 as usize;
            if time.1 {
                sleeps[index..].clone_from_slice(&awake[index..]);
            } else {
                sleeps[index..].clone_from_slice(&asleep[index..]);
            }
        }
        *guard += sleeps.iter().sum::<u32>();
    }

    let mut max_sleep = ("", 0u32);
    for (guard, sleeps) in &guards {
        if *sleeps > max_sleep.1 {
            max_sleep = (guard, *sleeps);
        }
    }

    let mut sleep_count = [0; 60];
    for (date, day) in &days {
        if day.guard == max_sleep.0 {
            let sleeps = day_sleeps[date];
            for i in 0..60 {
                sleep_count[i] += sleeps[i];
            }
        }
    }

    max_sleep.0.parse::<usize>().unwrap()
        * sleep_count
            .iter()
            .enumerate()
            .map(|(x, y)| (y, x))
            .max()
            .unwrap()
            .1
}

fn process_data_b(data: &str) -> u32 {
    let mut days = HashMap::new();
    for line in data.lines() {
        let record: Record = line.parse().unwrap();
        match record {
            Record::Started { date, guard } => {
                let day = days.entry(date).or_insert(WatchDay {
                    guard: "".to_owned(),
                    times: vec![],
                });
                day.guard = guard.clone();
            }
            Record::Switched {
                date,
                minute,
                awake,
            } => {
                let day = days.entry(date).or_insert(WatchDay {
                    guard: "".to_owned(),
                    times: vec![],
                });
                day.times.push((minute, awake));
                day.times.sort_unstable();
            }
        }
    }

    let awake = [0; 60];
    let asleep = [1; 60];
    let mut guards = HashMap::new();
    let mut day_sleeps = HashMap::new();
    for (date, day) in &days {
        let guard = guards.entry(&day.guard).or_insert(0);
        let sleeps = day_sleeps.entry(date).or_insert([0; 60]);
        for time in &day.times {
            let index = time.0 as usize;
            if time.1 {
                sleeps[index..].clone_from_slice(&awake[index..]);
            } else {
                sleeps[index..].clone_from_slice(&asleep[index..]);
            }
        }
        *guard += sleeps.iter().sum::<u32>();
    }

    let mut guard_sleeps = HashMap::new();
    for (date, day) in &days {
        let guard = guard_sleeps.entry(&day.guard).or_insert([0; 60]);
        let sleeps = day_sleeps.entry(date).or_insert([0; 60]);
        for i in 0..60 {
            guard[i] += sleeps[i];
        }
    }

    let mut max_sleep = ("", 0u32, 0u32);
    for (guard, sleep_count) in &guard_sleeps {
        let sleeps = sleep_count
            .iter()
            .enumerate()
            .map(|(x, y)| (y, x))
            .max()
            .unwrap();
        if *sleeps.0 > max_sleep.1 {
            max_sleep = (guard, *sleeps.0, sleeps.1 as u32);
        }
    }

    max_sleep.0.parse::<u32>().unwrap() * max_sleep.2
}

//-----------------------------------------------------
// Questions.

q_impl!("4");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"
        ),
        240
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"
        ),
        4455
    );
}
