extern crate rand;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use structopt::StructOpt;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short)]
    underscore: bool,

    #[structopt(default_value = "/usr/share/dict/words", parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let file = File::open(&args.path)
        .unwrap_or_else(|e| panic!("file não encontrado: {}: {}", &args.path.display(), e));

    let rnd_line = random_line(file);

    if args.underscore {
        let len = rnd_line.chars().count();
        let mut s = String::from(&rnd_line);
        let idx = len / 2;
        s.insert_str(idx, "_");
        println!("{}", s);
    } else {
        println!("{}", rnd_line);
    }
}

fn random_line(file: File) -> String {
    let f = BufReader::new(file);
    let lines = f.lines().map(|l| l.expect("não conseguiu ler a linha"));

    match rand::seq::sample_iter(&mut rand::thread_rng(), lines, 1) {
        Ok(mut v) => v.pop().unwrap(),
        Err(_) => panic!("File is empty"),
    }
}
