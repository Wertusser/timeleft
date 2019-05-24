use clap::{App, ArgMatches};
use clap::load_yaml;
use chrono::prelude::*;
use std::process::exit;
use colored::*;

const ERROR_STR: &str = "Bad string, I can't parse it! Use 24-hour format like 21:34:00 or 21:34";

fn parse_str_to_int(s: &str) -> u32 {
    match s.parse() {
        Ok(v) => v,
        Err(_) => panic!(ERROR_STR)
    }
}

fn parse_time(s: &str) -> Result<DateTime<Local>, &str> {
    let now: DateTime<Local> = Local::now();
    let dt: Vec<u32> = s.split(":").map(parse_str_to_int).collect();
    match dt.len() {
        2 => Ok(now.date().and_hms(dt[0], dt[1], 0)),
        3 => Ok(now.date().and_hms(dt[0], dt[1], dt[2])),
        _ => Err(ERROR_STR)
    }
}

fn print_left(left: u64, matches: &ArgMatches) {
    let time = NaiveTime::from_num_seconds_from_midnight(left as u32, 0);
    if matches.is_present("second") {
        print!("{} {} ", left.to_string().green(), "whole seconds left");
    } else if matches.is_present("minute") {
        print!("{} {} ", (left / 60).to_string().green(), "whole minutes left");
    } else if matches.is_present("hour") {
        print!("{} {} ", (left / 3600).to_string().green(), "whole hours left");
    } else if matches.is_present("all") {
        print!("{} ({} sec, {} min, {} hr) left ",
               time.to_string().yellow().bold(),
               left.to_string().green(),
               (left / 60).to_string().green(),
               (left / 3600).to_string().green());
    } else {
        print!("{} {} ", time.to_string().yellow().bold(), "left");
    };
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches: ArgMatches = App::from_yaml(yaml).get_matches();

    let now: DateTime<Local> = Local::now();
    let next_day = match matches.value_of("time") {
        Some(s) => parse_time(s).expect("Find error"),
        None => now.date().succ().and_hms(0, 0, 0)
    };
    if next_day < now {
        println!("{}", " THE DEADLINE HAS COME ".red().on_black().bold());
        exit(1);
    }
    let left = next_day.signed_duration_since(now).to_std().unwrap();
    print_left(left.as_secs(), &matches);

    match matches.value_of("time") {
        Some(_) => println!("{} {}", "until", next_day.time().to_string().blue()),
        None => println!("{} {}", "until", "next day.".blue())
    };
}
