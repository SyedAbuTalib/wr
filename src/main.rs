use std::{
    error::Error,
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Cli {
    Add {
        short_hand: String,
    },
    #[structopt(external_subcommand)]
    Other(Vec<String>),
}

fn main() {
    let cli = Cli::from_args();

    println!("cli is {:?}", cli);

    match Cli::from_args() {
        Cli::Add { short_hand } => add(&short_hand),
        Cli::Other(args) => warp(&args.concat()),
    };
}

fn get_file_path() -> Result<PathBuf, Box<dyn Error>> {
    let mut path = dirs::home_dir().unwrap();
    path.push(".wr");
    Ok(path)
}

fn add(short: &str) -> Result<String, Box<dyn Error>> {
    let path = get_file_path()?;
    let mut f = File::create(path)?;
    f.write(format!("{}:{}\n", short, std::env::current_dir().unwrap().display()).as_bytes())?;
    Ok("Added warp point".into())
}

fn warp(short: &str) -> Result<String, Box<dyn Error>> {
    let path = get_file_path().unwrap();
    let points = read_to_string(path).unwrap();

    for line in points.lines() {
        let mut parts = line.split(":");
        let s = parts.next().unwrap();
        let path = parts.next().unwrap();
        if s == short {
            println!("found the shorthand {}! Its path is {}", s, path);
            break;
        }
    }
    Ok("warped".into())
}
