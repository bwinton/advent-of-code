use std::{
    env::{var, VarError},
    fs::{self, copy, create_dir_all, read, read_dir, remove_file, File, OpenOptions},
    io::{stdout, Stdout, Write},
    time::SystemTimeError,
};

use chrono::{Datelike, Duration, Local};
use clap::{command, Arg, ArgAction};
use crossterm::{style::Print, ExecutableCommand};
use custom_error::custom_error;
use humantime::format_duration;
use reqwest::{
    blocking::{Client, Response},
    header::COOKIE,
};

custom_error! { NextError
    IOError{source: std::io::Error} = "io error",
    Request{source: reqwest::Error} = "request error",
    Time{source: SystemTimeError} = "time error",
    JSON{source: serde_json::error::Error} = "json error",
    YearsNotFound{} = "Years not found in directories.",
    YearNotFound{year:i32} = "Year {year} not found on server.",
    InputNotFound{year:i32, day:u32} = "Input \"https://adventofcode.com/{year}/day/{day}/input\" not found on server.",
    ParseIntError{source: std::num::ParseIntError} = "Couldn't parse int",
    // CrossTerm{source: crossterm::Error} = "crossterm error",
    FromUtf8Error{source: std::string::FromUtf8Error} = "Couldn't read input",
    ModulesNotFound{location: String} = "Modules {location} not found in main",
    Session{source: VarError} = "Could not find AOC_SESSION env variable.",
    DurationError{duration: Duration} = "Sleep Duration {duration} out of range.",
}

fn get_last() -> Result<(u32, String), NextError> {
    let mut entries = read_dir("src/")?
        .filter_map(|res| {
            let name = res.map(|e| e.file_name());
            let name = name.as_ref().ok()?.to_str()?;
            name.parse::<u32>().ok()
        })
        .collect::<Vec<_>>();
    entries.sort_unstable();

    let last_year = entries
        .into_iter()
        .last()
        .ok_or(NextError::YearsNotFound {})?;

    let mut entries = read_dir(format!("src/{}", last_year))?
        .filter_map(|res| {
            let name = res.map(|e| e.file_name());
            let name = name.as_ref().ok()?.to_str()?;
            if name.ends_with(".rs") {
                Some(name.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    entries.sort();
    let last_day = entries
        .into_iter()
        .last()
        .ok_or(NextError::YearsNotFound {})?;
    Ok((last_year, last_day))
}

fn add_day(last_year: u32, day: u32, stdout: &mut Stdout) -> Result<(), NextError> {
    stdout.execute(Print(format!("Adding Day: {}/{:02}\n", last_year, day)))?;
    File::create(format!("src/{}/data/q{:02}.data", last_year, day))?;

    // Copy the template, replacing "XX" with "<:02=day>", and "X" with "<day>".
    let template = String::from_utf8(read("template/qXX.rs")?)?;
    let template = template.replace("XX", &format!("{:02}", day));
    let template = template.replace('X', &format!("{}", day));

    let mut question = File::create(format!("src/{}/q{:02}.rs", last_year, day))?;
    question.write_all(template.as_bytes())?;

    // Edit main to add "mod q<:02=day>;" and ", q<:02=day>"…
    stdout.execute(Print(format!("Editing: {}/main.rs\n", last_year)))?;

    let main = String::from_utf8(read(format!("src/{}/main.rs", last_year))?)?;
    let index = main
        .find("// modules\n")
        .ok_or(NextError::ModulesNotFound {
            location: "import start".to_owned(),
        })?;
    let next = index
        + main[index..]
            .find("\n\n")
            .ok_or(NextError::ModulesNotFound {
                location: "import end".to_owned(),
            })?
        + 1;
    let main = format!("{}mod q{:02};\n{}", &main[..next], day, &main[next..]);

    let index = main.find("q_vec!(").ok_or(NextError::ModulesNotFound {
        location: "qvec start".to_owned(),
    })?;
    let next = index
        + main[index..]
            .find(");\n")
            .ok_or(NextError::ModulesNotFound {
                location: "qvec end".to_owned(),
            })?;
    let main = format!("{} q{:02},{}", &main[..next], day, &main[next..]);

    let mut main_out = File::create(format!("src/{}/main.rs", last_year))?;
    main_out.write_all(main.as_bytes())?;

    Ok(())
}

fn add_year(year: u32, stdout: &mut Stdout) -> Result<(), NextError> {
    stdout.execute(Print(format!("Adding Year: {}\n", year)))?;
    create_dir_all(format!("src/{}/data", year))?;
    copy("template/main.rs", format!("src/{}/main.rs", year))?;
    add_day(year, 1, stdout)?;
    Ok(())
}

fn add_next(last_year: u32, last_day: &str, stdout: &mut Stdout) -> Result<(), NextError> {
    stdout.execute(Print("Figuring out what to do…\n"))?;
    if last_day != "q25.rs" {
        let last_day = &last_day[1..3];
        let last_day = last_day.parse::<u32>()?;
        add_day(last_year, last_day + 1, stdout)?;
    } else {
        add_year(last_year + 1, stdout)?;
    }

    Ok(())
}

fn get_url(year: i32, day: u32) -> Result<Response, NextError> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let mut request = Client::new().get(url);
    let session_cookie = format!("session={}", &var("AOC_SESSION")?);
    request = request.header(COOKIE, session_cookie);
    let response = request.send()?;
    Ok(response)
}

fn get_all_inputs() -> Result<(), NextError> {
    let today = Local::now().naive_local();
    let current_year = today.year();
    let current_day = today.day();

    for year in 2015..current_year {
        create_dir_all(format!("src/{}/data", year))?;
        for day in 1..=25 {
            if download_input(year, day)? {
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        }
    }
    let year = current_year;
    create_dir_all(format!("src/{}/data", year))?;
    for day in 1..=current_day {
        if download_input(year, day)? {
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
    Ok(())
}

fn download_input(year: i32, day: u32) -> Result<bool, NextError> {
    let datapath = format!("src/{}/data/q{:02}.data", year, day);
    let metadata = std::fs::metadata(&datapath);
    if metadata.map(|m| m.len()).unwrap_or(1) == 0 {
        // If the file is empty, delete it so that we can re-download it!
        fs::remove_file(&datapath)?;
    }
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&datapath);
    Ok(if let Ok(mut file) = file {
        println!("Getting input for {:?}", datapath);
        let response = get_url(year, day);
        if response.is_err() {
            let rm = remove_file(&datapath);
            rm?;
        }
        let mut response = response?;
        if !response.status().is_success() {
            let rm = remove_file(&datapath);
            dbg!(&rm);
            rm?;
        }
        response.copy_to(&mut file)?;
        true
    } else {
        println!("Already have {:?}", datapath);
        false
    })
}

fn get_input() -> Result<(), NextError> {
    let today = Local::now().naive_local();
    let year = today.year();
    let day = today.day();
    let datapath = format!("src/{}/data/q{:02}.data", year, day);
    let metadata = std::fs::metadata(datapath);
    if metadata.map(|m| m.len()).unwrap_or(1) == 0 {
        // We're getting the data for today!
        download_input(year, day)?;
    } else {
        // Let's wait and get the data for tomorrow!
        let tomorrow = today
            .date()
            .succ_opt()
            .ok_or(NextError::YearNotFound { year })?;
        let day = tomorrow.day();
        let datapath = format!("src/{}/data/q{:02}.data", year, day);
        println!("Getting input for {:?}", datapath);
        let tonight = tomorrow.and_hms_opt(0, 0, 1).unwrap();
        let duration = tonight - today;
        let std_duration = std::time::Duration::from_secs(
            duration
                .to_std()
                .map_err(|_| NextError::DurationError { duration })?
                .as_secs(),
        );
        println!(
            "  Sleeping for {} until {}",
            format_duration(std_duration),
            tonight
        );
        std::thread::sleep(std_duration);
        println!("  Waking up and getting the response!");
        let mut response = get_url(year, day)?;
        println!("  {:?}", response.status());
        if !response.status().is_success() {
            // try again in 5 seconds…
            println!("  Trying again in {:?}!", Duration::seconds(5).to_std());
            std::thread::sleep(
                Duration::seconds(5)
                    .to_std()
                    .map_err(|_| NextError::DurationError { duration })?,
            );
            println!("  Waking up and getting the response!");
            response = get_url(year, day)?;
            println!("  {:?}", response.status());
            if !response.status().is_success() {
                return Err(NextError::InputNotFound { year, day });
            }
        }
        let mut file = File::create(&datapath)?;
        response.copy_to(&mut file)?;
    }
    Ok(())
}

fn main() -> Result<(), NextError> {
    let mut stdout = stdout();
    color_backtrace::install();
    let matches = command!("\n")
        .arg(
            Arg::new("year")
                .short('y')
                .help("Which year to add")
                .long_help("Specify a year to add.")
                .group("arg"),
        )
        .arg(
            Arg::new("day")
                .short('d')
                .help("Which day to add")
                .long_help("Specify a day to add.")
                .group("arg"),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .help("Get input")
                .long_help("Get input for today.")
                .action(ArgAction::SetTrue)
                .group("arg"),
        )
        .arg(
            Arg::new("all_inputs")
                .short('a')
                .help("Get all inputs")
                .long_help("Get all the missing input.")
                .action(ArgAction::SetTrue)
                .group("arg"),
        )
        .get_matches();

    let (last_year, last_day) = get_last()?;

    if let Some(&day) = matches.get_one::<u32>("day") {
        add_day(last_year, day, &mut stdout)?;
    } else if let Some(&year) = matches.get_one::<u32>("year") {
        add_year(year, &mut stdout)?;
    } else if *matches.get_one("input").unwrap() {
        get_input()?;
    } else if *matches.get_one("all_inputs").unwrap() {
        get_all_inputs()?;
    } else {
        add_next(last_year, &last_day, &mut stdout)?;
    }

    stdout.flush()?;
    Ok(())
}
