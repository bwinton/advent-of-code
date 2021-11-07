use std::{
    fs::{copy, create_dir_all, read, read_dir, File},
    io::{stdout, Stdout, Write},
    time::SystemTimeError,
};

use clap::{app_from_crate, Arg};
use crossterm::{style::Print, ExecutableCommand};
use custom_error::custom_error;

custom_error! { NextError
    Request{source: reqwest::Error} = "request error",
    Time{source: SystemTimeError} = "time error",
    JSON{source: serde_json::error::Error} = "json error",
    YearNotFound{} = "Years not found in directories.",
    ParseIntError{source: std::num::ParseIntError} = "Couldn't parse int",
    CrossTerm{source: crossterm::ErrorKind} = "crossterm error",
    FromUtf8Error{source: std::string::FromUtf8Error} = "Couldn't read input",
    ModulesNotFound{location: String} = "Modules {location} not found in main",
}

fn get_last() -> Result<(u32, String), NextError> {
    let mut entries = read_dir("src/")?
        .filter_map(|res| {
            let name = res.map(|e| e.file_name());
            let name = name.as_ref().ok()?.to_str()?;
            name.parse::<u32>().ok()
        })
        .collect::<Vec<_>>();
    entries.sort();

    let last_year = entries
        .into_iter()
        .last()
        .ok_or(NextError::YearNotFound {})?;

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
        .ok_or(NextError::YearNotFound {})?;
    Ok((last_year, last_day.to_owned()))
}

fn add_day(last_year: u32, day: u32, stdout: &mut Stdout) -> Result<(), NextError> {
    stdout.execute(Print(format!("Adding Day: {}/{:02}\n", last_year, day)))?;
    File::create(format!("src/{}/data/q{:02}.data", last_year, day))?;

    // Copy the template, replacing "XX" with "<:02=day>", and "X" with "<day>".
    let template = String::from_utf8(read("template/qXX.rs")?)?;
    let template = template.replace("XX", &format!("{:02}", day));
    let template = template.replace("X", &format!("{}", day));

    let mut question = File::create(format!("src/{}/q{:02}.rs", last_year, day))?;
    question.write(&template.as_bytes())?;

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
    let main = format!("{}q{:02},{}", &main[..next], day, &main[next..]);

    let mut main_out = File::create(format!("src/{}/main.rs", last_year))?;
    main_out.write(&main.as_bytes())?;

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
    stdout.execute(Print(format!("Figuring out what to do…\n")))?;
    if last_day != "q25.rs" {
        let last_day = &last_day[1..3];
        let last_day = last_day.parse::<u32>()?;
        add_day(last_year, last_day + 1, stdout)?;
    } else {
        add_year(last_year + 1, stdout)?;
    }

    Ok(())
}

fn main() -> Result<(), NextError> {
    let mut stdout = stdout();
    color_backtrace::install();
    let matches = app_from_crate!("\n")
        .arg(
            Arg::new("year")
                .short('y')
                .about("Which year to add")
                .long_about("Specify a year to add.")
                .takes_value(true)
                .group("input"),
        )
        .arg(
            Arg::new("day")
                .short('d')
                .about("Which day to add")
                .long_about("Specify a day to add.")
                .takes_value(true)
                .group("input"),
        )
        .get_matches();

    let (last_year, last_day) = get_last()?;

    if let Some(day) = matches.value_of("day") {
        add_day(last_year, day.parse::<u32>()?, &mut stdout)?;
    } else if let Some(year) = matches.value_of("year") {
        add_year(year.parse::<u32>()?, &mut stdout)?;
    } else {
        add_next(last_year, &last_day, &mut stdout)?;
    }

    stdout.flush()?;
    Ok(())
}
