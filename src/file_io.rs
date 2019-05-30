extern crate chrono;

use chrono::{Date, Utc};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader};

static FILENAME: &str = "reading.csv";

pub fn initialise_file() -> std::io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(FILENAME)?;

    let mut line = String::new();
    let mut buffered_reader = BufReader::new(file);

    if buffered_reader
        .read_line(&mut line)
        .expect("unable to read line")
        == 0
    {
        let mut file = OpenOptions::new().append(true).open(FILENAME)?;
        file.write_fmt(format_args!(
            "index,when,book,author,start date, end date, motivation, concluding thoughts\n",
        ))?;
    }

    Ok(())
}

pub fn write_present_reading(
    book_title: &str,
    author: &str,
    start_date: &str,
    end_date: &str,
    motivation: &str,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(FILENAME)?;

    file.write_fmt(format_args!(
        "current,{},{},{},{},{},\n",
        book_title, author, start_date, end_date, motivation
    ))?;

    Ok(())
}

pub fn write_future_reading(
    book_title: &str,
    author: &str,
    motivation: &str,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(FILENAME)?;

    file.write_fmt(format_args!(
        "future,{},{},,,{},\n",
        book_title, author, motivation,
    ))?;

    Ok(())
}

pub fn write_past_reading(
    book_title: &str,
    author: &str,
    start_date: &str,
    end_date: Date<Utc>,
    thoughts: &str,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(FILENAME)?;

    file.write_fmt(format_args!(
        "finished,{},{},{},{},,{},\n",
        book_title, author, start_date, end_date, thoughts
    ))?;

    Ok(())
}

pub fn find_in_file(word: String) -> (String, String) {
    let buffered_reader = BufReader::new(File::open(FILENAME).expect("Cannot open file.txt"));

    let lines = buffered_reader.lines();

    let matching_line = lines
        .map(|l| l.unwrap())
        .filter(|line| line.contains(word.trim()))
        .next();

    (matching_line.unwrap(), word)
}
