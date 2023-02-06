use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "file", short, long)]
    file: String,
    #[arg(value_name = "num", short, long)]
    number: i32,
}

fn main() {
    let cli = Cli::parse();

    println!("The file passed is : {} ", cli.file);
    println!("Your favorite number must be {}.", cli.number + 5);
}
