use clap::Parser;
use std::{fs, io::Write, path::PathBuf};

#[derive(Parser, Debug)]
struct Args {
    /// .env file path, defaults to ".env"
    #[arg(short, long, default_value = ".env")]
    env: PathBuf,

    /// Output Directory path
    #[arg(short, long)]
    out: PathBuf,
}
fn main() {
    let args = Args::parse();

    let env_iter = dotenv::from_path_iter(args.env).expect("Could not parse .env file");

    fs::create_dir_all(&args.out).expect("Could not create output directory");
    for line in env_iter {
        let (key, value) = line.unwrap();
        let mut file = fs::File::create(args.out.join(key.to_uppercase())).unwrap();
        write!(file, "{value}").unwrap();
    }
}
