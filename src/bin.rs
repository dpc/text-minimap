extern crate text_minimap;
extern crate clap;

use std::{fs, io};

use text_minimap::*;

use clap::{Arg, App};

fn main() {
    let arguments = App::new("text-minimap")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Dawid Ciężarkiewicz <dpc@dpc.pw>")
        .about("Convert text to a minimap consisting of Unicode braille characters")
        .arg(Arg::with_name("FILE")
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .help("File to read input from, instead of stdin"))
        .arg(Arg::with_name("OUT_FILE")
                 .short("o")
                 .long("output")
                 .takes_value(true)
                 .help("File to write output to, instead of stdout"))
        .arg(Arg::with_name("XSCALE")
                 .short("x")
                 .long("xscale")
                 .takes_value(true)
                 .help("Number of characters width-wise to represent as a single dot"))
        .arg(Arg::with_name("YSCALE")
                 .short("y")
                 .long("yscale")
                 .takes_value(true)
                 .help("Number of characters height-wise to represent as a single dot"))
        .get_matches();

    let mut settings = Settings::new();

    settings.xscale = arguments
        .value_of("XSCALE")
        .unwrap_or("1")
        .parse::<usize>()
        .unwrap();

    settings.yscale = arguments
        .value_of("YSCALE")
        .unwrap_or("1")
        .parse::<usize>()
        .unwrap();

    let stdin = std::io::stdin();
    let input: Box<io::Read> = if let Some(in_path) = arguments.value_of("FILE") {
        let file = fs::File::open(in_path).unwrap();

        Box::new(file)
    } else {
        let stdin = stdin.lock();

        Box::new(stdin)
    };

    let mut output: Box<io::Write> = if let Some(out_path) = arguments.value_of("OUT_FILE") {
        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(out_path)
            .unwrap();
        Box::new(file)
    } else {
        let stdout = std::io::stdout();
        Box::new(stdout)
    };

    for line in to_minimap(input, settings) {
        writeln!(output, "{}", line).unwrap();
    }
}
