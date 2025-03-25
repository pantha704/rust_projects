extern crate clap;

use clap::{Arg, App};
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("Panther <prathamjaiswal204@gmail.com> ")
        .about("wget clone written in Rust!")
        .arg(Arg::with_name("URL")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("url to download"))
        .get_matches();

    let url = matches.value_of("URL").unwrap();
    println!{"{}", url};
}

fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => {
            match length {
                Some(len) => ProgressBar::new(len),
                None => ProgressBar::new_spinner(),
            }
        }
    };


    bar.set_message(msg.to_string());
    match length.is_some() {
        true => bar
        .set_style(ProgressStyle::default_bar()
            .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}").expect("Failed to set style")
            .progress_chars("=> ")),
        false => bar.set_style(ProgressStyle::default_spinner()),
        };

    bar
}