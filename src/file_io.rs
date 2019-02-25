extern crate chrono;

use chrono::{Date, Utc};
use std::fs::OpenOptions;
use std::io::Write;

static FILENAME: &str = "reading.txt";

pub fn initialise_file() -> std::io::Result<()> {
    OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(FILENAME)?;

    Ok(())
}

pub fn write_present_reading(
    book_title: &str,
    author: &str,
    start_date: &str,
    end_date: &str,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(FILENAME)?;

    file.write_fmt(format_args!(
        "Reading {:?}, by {:?}. Started: {:?} and ended: {:?}\n",
        book_title, author, start_date, end_date
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
        "Future reading {:?}, by {:?}, because: {:?}\n",
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
        "Finished reading {:?}, by {:?}. Started: {:?} and ended: {:?}. Concluding thoughts: {:?}\n",
        book_title, author, start_date, end_date, thoughts
    ))?;

    Ok(())
}

pub fn find_in_file(book_title: String, contents: &str) -> (&str, String) {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(&book_title.trim()) {
            results.push(line);
        }
    }

    (results.first().expect("Empty array"), book_title)
}
