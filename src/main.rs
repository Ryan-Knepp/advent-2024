use clap::Parser;
mod day1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Which day to run
    #[arg(short, long, default_value_t = 1)]
    day: u8,

    /// File name of input
    #[arg(short, long, default_value_t = String::from("input.txt"))]
    input: String,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day1::run(&args.input),
        _ => println!("Day {} not implemented", args.day),
    }
}
