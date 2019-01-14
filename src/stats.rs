use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use custom_error::custom_error;
use reqwest;
use serde_json::{from_reader, Map, Value};
use std::env::{var, VarError};
use std::fs::File;
use std::path::Path;
use std::time::{SystemTime, SystemTimeError};

static ONE_DAY_IN_SECS: u64 = 24 * 60 * 60;

custom_error! { StatsError
    Request{source: reqwest::Error} = "request error",
    Cache{source: std::io::Error} = "invalid cache file",
    Time{source: SystemTimeError} = "time error",
    JSON{source: serde_json::error::Error} = "json error",
    YearNotFound{year:String} = "Year {year} not found on server.",
    Session{source: VarError} = "Could not find AOC_SESSION env variable.",
}

#[derive(Debug)]
struct AocStats {
    owner_id: String,
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
            owner_id: data["owner_id"].as_str().unwrap().to_owned(),
            members,
            event: data["event"].as_str().unwrap().to_owned(),
        }
    }
}

#[derive(Debug)]
struct Member {
    name: String,
    completions: [(Option<i64>, Option<i64>); 25],
    stars: i64,
    id: String,
    global_score: i64,
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
                    .map(|x| x["get_star_ts"].as_str().unwrap().parse().unwrap());
                let two = completion
                    .get("2")
                    .map(|x| x["get_star_ts"].as_str().unwrap().parse().unwrap());
                day_level.0 = one;
                day_level.1 = two;
            }
        }

        Member {
            name: data["name"].as_str().unwrap().to_owned(),
            completions,
            stars: data["stars"].as_i64().unwrap_or(0),
            id: data["id"].as_str().unwrap().to_owned(),
            global_score: data["global_score"].as_i64().unwrap_or(0),
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

fn print_year(year: &str) -> Result<(), StatsError> {
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
        println!("Cache doesn't exist or is too old. Downloading…");
        let mut request = reqwest::Client::new().get(&stats_url);
        request = request.header(reqwest::header::COOKIE, session_cookie);

        let mut response = request.send()?;
        if !response.status().is_success() {
            return Err(StatsError::YearNotFound {
                year: year.to_owned(),
            });
        }
        let mut file = File::create(&cache_name)?;
        response.copy_to(&mut file)?;
    }
    let cache_file = File::open(&cache_name)?;
    let data: Map<String, Value> = from_reader(cache_file)?;
    let mut stats = AocStats::from_data(&data);
    println!("Done {}!", stats.event);
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
        println!(
            "  {}: {} -> {} ({:+})",
            member.name,
            member.local_score,
            member.max_score,
            member.place.unwrap() - i as i64
        );
    }
    println!();
    Ok(())
}

fn main() {
    let matches = app_from_crate!("\n")
        .arg(
            Arg::with_name("year")
                .help("Which year(s) to run")
                .long_help(
                    "Specify a year, or years to run.
 Putting nothing will run all years.
",
                )
                .index(1)
                .multiple(true)
                .default_value("*"),
        )
        .get_matches();

    let mut args: Vec<&str> = matches.values_of("year").unwrap().collect();
    args = args
        .iter()
        .map(|&x| {
            if x == "*" {
                vec!["2015", "2016", "2017", "2018"]
            } else {
                vec![x]
            }
        })
        .flatten()
        .collect();
    // println!("args: {:?}", args);

    for year in args {
        if let Err(error) = print_year(year) {
            println!("{}", error);
        };
    }
}
