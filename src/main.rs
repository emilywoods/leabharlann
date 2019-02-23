use std::env;
use std::fs::OpenOptions;
use std::io::{stdin, stdout, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = parse_config(&args);

    match query {
        "now" | "anois" => now(),
        "finish" | "criochnaigh" => finish(),
        "future" | "sa-todhchai" => future(),
        _ => println!("What are you reading?"),
    }
}

fn parse_config(args: &[String]) -> (&str) {
    let query = &args[1];
    query
}

fn now() {
    let (book_title, author, start_date, end_date) = user_interaction_reading();

    let book_title = book_title.trim();
    let author = author.trim();
    let start_date = start_date.trim();
    let end_date = end_date.trim();

    println!(
        "Reading {:?}, by {:?}. Started: {:?} and ended: {:?}",
        book_title, author, start_date, end_date
    );

    write_to_file_reading(book_title, author, start_date, end_date).expect("error writing to file");
}

fn finish() {
    println!("What am I finishing?")
}

fn future() {
    let (book_title, author, motivation) = user_interaction_future();

    let book_title = book_title.trim();
    let author = author.trim();
    let motivation = motivation.trim();

    println!("Future reading: {:?}, by {:?}", book_title, author);

    write_to_file_future(book_title, author, motivation).expect("error writing to file");
}

fn user_interaction_reading() -> (String, String, String, String) {
    let mut book_title = String::new();
    let mut author = String::new();
    let mut start_date = String::new();
    let mut end_date = String::new();

    let _ = stdout().flush(); // what is the purpose of this?

    println!("What are you reading?");
    stdin()
        .read_line(&mut book_title)
        .expect("Failed to read line");

    println!("Who wrote it?");
    stdin().read_line(&mut author).expect("Failed to read line");

    println!("When did you start?");
    stdin()
        .read_line(&mut start_date)
        .expect("Failed to read line");

    println!("When did you finish?");
    stdin()
        .read_line(&mut end_date)
        .expect("Failed to read line");

    (book_title, author, start_date, end_date)
}

fn user_interaction_future() -> (String, String, String) {
    let mut book_title = String::new();
    let mut author = String::new();
    let mut motivation = String::new();

    let _ = stdout().flush(); // what is the purpose of this?

    println!("What book do you want to read?");
    stdin()
        .read_line(&mut book_title)
        .expect("Failed to read line");

    println!("Who wrote it?");
    stdin().read_line(&mut author).expect("Failed to read line");

    println!("Why do you want to read it?");
    stdin()
        .read_line(&mut motivation)
        .expect("Failed to read line");

    (book_title, author, motivation)
}

fn write_to_file_reading(
    book_title: &str,
    author: &str,
    start_date: &str,
    end_date: &str,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("reading.txt")?;

    file.write_fmt(format_args!(
        "Reading {:?}, by {:?}. Started: {:?} and ended: {:?}\n",
        book_title, author, start_date, end_date
    ))?;

    Ok(())
}

fn write_to_file_future(book_title: &str, author: &str, motivation: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("reading.txt")?;

    file.write_fmt(format_args!(
        "\nFuture reading {:?}, by {:?}, because: {:?}\n",
        book_title, author, motivation,
    ))?;

    Ok(())
}
