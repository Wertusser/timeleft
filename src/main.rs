use clap::{App, ArgMatches};
use clap::load_yaml;
use chrono::prelude::*;
use std::process::exit;
use colored::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches: ArgMatches = App::from_yaml(yaml).get_matches();

    let now: DateTime<Local> = Local::now();
    let next_day = match matches.value_of("time") {
        Some(s) => {
            let dt: Vec<u32> = s.split(":").map(|x| x.parse().unwrap_or(0)).collect();
            match dt.len() {
                2 => now.date().and_hms(dt[0], dt[1], 0),
                3 => now.date().and_hms(dt[0], dt[1], dt[2]),
                _ => now.date().succ().and_hms(0, 0, 0)
            }
        }
        None => now.date().succ().and_hms(0, 0, 0)
    };
    if next_day < now {
        println!("{}", " THE DEADLINE HAS COME ".red().on_black().bold());
        exit(1);
    }
    let left = next_day.signed_duration_since(now).to_std().unwrap();
    let t = NaiveTime::from_num_seconds_from_midnight(left.as_secs() as u32, 0);
    if matches.is_present("second") {
        print!("{} {} ", left.as_secs().to_string().green(), "whole seconds left");
    } else if matches.is_present("minute") {
        print!("{} {} ", (left.as_secs() / 60).to_string().green(), "whole minutes left");
    } else if matches.is_present("hour") {
        print!("{} {} ", (left.as_secs() / 3600).to_string().green(), "whole hours left");
    } else if matches.is_present("all") {
        print!("{} ({} sec, {} min, {} hr) left ",
               t.to_string().yellow().bold(),
               left.as_secs().to_string().green(),
               (left.as_secs() / 60).to_string().green(),
               (left.as_secs() / 3600).to_string().green());
    } else {
        print!("{} {} ", t.to_string().yellow().bold(), "left");
    };

    match matches.value_of("time") {
        Some(s) => println!("{} {}", "until", s.blue()),
        None => println!("{} {}", "until", "next day.".blue())
    };
}
