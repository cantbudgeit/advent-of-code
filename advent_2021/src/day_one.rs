use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use wfd::open_dialog;

pub fn run_question_1() -> String {
    let dialog_result = open_dialog(Default::default()).unwrap();
    let lines = lines_from_file(dialog_result.selected_file_path);

    let mut increase_count = 0;
    let mut last = "";
    for line in lines.iter() {
        if last != "" {
            if line.parse::<i32>().unwrap() > last.parse::<i32>().unwrap() {
                increase_count = &increase_count + 1;
                println!(
                    "{} increased -- count:{}",
                    line.parse::<i32>().unwrap(),
                    increase_count
                );
            }
            println!(
                "{} same -- count:{}",
                line.parse::<i32>().unwrap(),
                increase_count
            );
        }
        last = line;
    }
    return increase_count.to_string();
}

pub fn run_question_2() -> String {
    let dialog_result = open_dialog(Default::default()).unwrap();
    let lines = lines_from_file(dialog_result.selected_file_path);

    let mut increase_count = 0;
    let mut last = "";
    for line in lines.iter() {
        if last != "" {
            if line.parse::<i32>().unwrap() > last.parse::<i32>().unwrap() {
                increase_count = &increase_count + 1;
                println!(
                    "{} increased -- count:{}",
                    line.parse::<i32>().unwrap(),
                    increase_count
                );
            }
            println!(
                "{} same -- count:{}",
                line.parse::<i32>().unwrap(),
                increase_count
            );
        }
        last = line;
    }
    return increase_count.to_string();
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
