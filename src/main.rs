use std::env;
use std::fs::File;
use std::io::{stdin,stdout,Write};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];


    let mut book_title = String::new();
    let mut author = String::new();
    let mut start_date = String::new();
    let mut end_date= String::new();

    let _=stdout().flush(); // what is the purpose of this?

    println!("What are you reading?");
    stdin().read_line(&mut book_title).expect("Please enter a book title");

    println!("Who wrote it?");
    stdin().read_line(&mut author).expect("Please enter the author(s)");

    println!("When did you start?");
    stdin().read_line(&mut start_date).expect("Please enter time you started");

    println!("When did you finish?");
    stdin().read_line(&mut end_date).expect("No end date entered. The book is marked as 'in-progress'");

    let book_title = book_title.trim();
    let author = author.trim();
    let start_date = start_date.trim();
    let end_date= end_date.trim();

    println!("Reading {:?}, by {:?}. Started: {:?} and ended: {:?}", book_title.trim(), author.trim(), start_date.trim(), end_date.trim());

    let mut file = File::create("reading.txt")?;

    file.write_fmt(format_args!("Reading {:?}, by {:?}. Started: {:?} and ended: {:?}", book_title, author, start_date, end_date))?;

    Ok(())
}
