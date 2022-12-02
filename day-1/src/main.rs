use std::fs::File;
use std::io::{self, BufRead, Read};
use std::num::ParseIntError;
use std::str::FromStr;


use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// File to parse
   #[arg(short, long)]
   filename: String,
}

#[derive(Debug)]
enum ParseError {
    Io(io::Error),
    Parse,
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError::Io(err)
    }
}

fn main() -> Result<(), ParseError> {

    let args = Args::parse();

    let mut file = File::open(&args.filename)?;

    let mut contents = io::BufReader::new(&mut file).bytes().flatten();

    let folded: (i32, i32, i32) = contents.try_fold((0, 0, 0), |(max, acc, curr_int), c| {
        let c = c as char;
        match c {
            '\n' => if curr_int == 0 { Ok((max.max(acc), 0, 0)) } else { Ok((max, curr_int + acc, 0)) },
            '0'..='9' => Ok((max, acc, curr_int * 10 + c as i32 - '0' as i32)),
            _ => Err(ParseError::Parse),
        }
    })?;

    let max = folded.0;

    println!("The elf carrying the most calories carries {} calories", max);
    Ok(())
}
