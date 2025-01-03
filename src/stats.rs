use std::{
    cmp::Ordering,
    env::{VarError, var},
    fs::{File, remove_file},
    io::{Stdout, Write, stdout},
    path::Path,
    time::{SystemTime, SystemTimeError},
};

use chrono::{Datelike, Local};
use clap::{Arg, command};
use crossterm::{
    ExecutableCommand, QueueableCommand, queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use custom_error::custom_error;

use serde_json::{Map, Value, from_reader};

static ONE_DAY_IN_SECS: u64 = 24 * 60 * 60;

custom_error! { StatsError
    IOError{source: std::io::Error} = "io error",
    Request{source: reqwest::Error} = "request error",
    // Cache{source: std::io::Error} = "invalid cache file",
    Time{source: SystemTimeError} = "time error",
    JSON{source: serde_json::error::Error} = "json error",
    YearNotFound{year:i32} = "Year {year} not found on server.",
    Session{source: VarError} = "Could not find AOC_SESSION env variable.",
    // CrossTerm{source: crossterm::ErrorKind} = "crossterm error",
}

#[derive(Debug)]
struct AocStats {
    _owner_id: String,
    members: Vec<Member>,
    event: String,
}

impl AocStats {
    fn from_data(data: &Map<String, Value>) -> Self {
        let mut members = vec![];
        for member in data["members"].as_object().unwrap().values() {
            members.push(Member::from_data(member.as_object().unwrap()));
        }
        AocStats {
            _owner_id: data["owner_id"].to_string(),
            members,
            event: data["event"].as_str().unwrap().to_owned(),
        }
    }
}

#[derive(Debug)]
struct Member {
    name: String,
    completions: [(Option<i64>, Option<i64>); 25],
    _stars: i64,
    _id: String,
    _global_score: i64,
    local_score: i64,
    max_score: i64,
    place: Option<i64>,
}

impl Member {
    fn from_data(data: &Map<String, Value>) -> Self {
        let mut completions = [(None, None); 25];
        for (i, day_level) in completions.iter_mut().enumerate() {
            let day = (i + 1).to_string();
            if let Some(completion) = &data["completion_day_level"].as_object().unwrap().get(&day) {
                let one = completion
                    .get("1")
                    .map(|x| x["get_star_ts"].as_i64().unwrap());
                let two = completion
                    .get("2")
                    .map(|x| x["get_star_ts"].as_i64().unwrap());
                day_level.0 = one;
                day_level.1 = two;
            }
        }

        Member {
            name: data["name"]
                .as_str()
                .unwrap_or(&format!("Anonymous User ({})", data["id"]))
                .to_owned(),
            completions,
            _stars: data["stars"].as_i64().unwrap_or(0),
            _id: data["id"].to_string(),
            _global_score: data["global_score"].as_i64().unwrap_or(0),
            local_score: data["local_score"].as_i64().unwrap_or(0),
            max_score: 0,
            place: None,
        }
    }

    fn calculate_max_score(&mut self, undone: &[(i64, i64); 25]) {
        self.max_score += self.local_score;
        for (remaining, completion) in undone.iter().enumerate().take(self.completions.len()) {
            if self.completions[remaining].0.is_none() {
                self.max_score += completion.0;
            }
            if self.completions[remaining].1.is_none() {
                self.max_score += completion.1;
            }
        }
    }
}

fn print_stats(stdout: &mut Stdout, stats: &mut AocStats) -> Result<(), StatsError> {
    queue!(
        stdout,
        Print("Stats for "),
        SetForegroundColor(Color::White),
        SetAttribute(Attribute::Bold),
        Print(stats.event.clone()),
        SetAttribute(Attribute::Reset),
        // ResetColor,
        Print(":\n")
    )?;
    let mut undone = [(0, 0); 25];
    for member in &stats.members {
        for (i, undone) in undone.iter_mut().enumerate().take(member.completions.len()) {
            if member.completions[i].0.is_none() {
                undone.0 += 1;
            }
            if member.completions[i].1.is_none() {
                undone.1 += 1;
            }
        }
    }
    stats.members.sort_by_key(|m| -m.local_score);
    for (i, member) in stats.members.iter_mut().enumerate() {
        member.calculate_max_score(&undone);
        member.place = Some(i as i64);
    }
    stats.members.sort_by_key(|m| -m.max_score);
    for (i, member) in stats.members.iter().enumerate() {
        let place = member.place.unwrap() - i as i64;
        let place_color = match place.cmp(&0) {
            Ordering::Greater => Color::Green,
            Ordering::Less => Color::Red,
            Ordering::Equal => Color::White,
        };

        queue!(
            stdout,
            SetForegroundColor(Color::Blue),
            Print(format!("  {}: ", member.name)),
            ResetColor,
            Print(format!("{} -> {} (", member.local_score, member.max_score)),
            SetForegroundColor(place_color),
            Print(format!("{}", place)),
            ResetColor,
            Print(")\n")
        )?;
    }
    stdout.queue(Print("\n"))?;
    Ok(())
}

fn print_year(year: i32, stdout: &mut Stdout) -> Result<(), StatsError> {
    let cache_name = format!("stats.{}.json", year);
    let stats_url = format!(
        "https://adventofcode.com/{}/leaderboard/private/view/70644.json",
        year
    );
    let cache = Path::new(&cache_name);
    let now = SystemTime::now();
    let session_cookie = format!("session={}", &var("AOC_SESSION")?);
    if !cache.is_file()
        || now.duration_since(cache.metadata()?.modified()?)?.as_secs() > ONE_DAY_IN_SECS
    {
        stdout.queue(Print("Cache doesn't exist or is too old. Downloading\n"))?;
        let mut request = reqwest::blocking::Client::new().get(stats_url);
        request = request.header(reqwest::header::COOKIE, session_cookie);

        let mut response = request.send()?;
        if !response.status().is_success() {
            return Err(StatsError::YearNotFound { year });
        }
        // dbg!(&response.text());
        let mut file = File::create(&cache_name)?;
        response.copy_to(&mut file)?;
    }
    let cache_file = File::open(&cache_name)?;
    let parsed_data = from_reader(cache_file);
    if parsed_data.is_err() {
        remove_file(cache)?;
        return Err(StatsError::from(parsed_data.err().unwrap()));
    }
    let data: Map<String, Value> = parsed_data?;
    let mut stats = AocStats::from_data(&data);
    print_stats(stdout, &mut stats)
}

fn main() -> Result<(), StatsError> {
    let mut stdout = stdout();
    color_backtrace::install();
    let matches = command!("\n")
        .arg(
            Arg::new("year")
                .help("Which year(s) to run")
                .long_help(
                    "Specify a year, or years to run.
 Putting nothing will run all years.
",
                )
                .index(1)
                .num_args(0..)
                .default_value("*"),
        )
        .get_matches();

    let args: Vec<&String> = matches.get_many("year").unwrap().collect();
    let today = Local::now().date_naive();
    let this_year = today.year();

    let args: Vec<i32> = args
        .iter()
        .flat_map(|&x| {
            if x == "*" {
                2015..=this_year
            } else {
                x.parse().unwrap()..=x.parse().unwrap()
            }
        })
        .collect();
    stdout.execute(Print(format!("args: {:?}\n", args)))?;

    for year in args {
        if let Err(error) = print_year(year, &mut stdout) {
            queue!(
                stdout,
                SetForegroundColor(Color::Red),
                Print(format!("{}", error)),
                ResetColor
            )?;
        };
        stdout.flush()?;
    }
    Ok(())
}
