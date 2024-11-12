use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use githubchart::{Chart, COLOR_SCHEMES};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <output_file> [-u <username>] [-i <input_file>] [-c <color_scheme>]", args[0]);
        std::process::exit(1);
    }

    let mut username = None;
    let mut input_file = None;
    let mut output_file = &args[1];
    let mut color_scheme = None;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-u" => {
                if i + 1 < args.len() {
                    username = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Missing value for -u");
                    std::process::exit(1);
                }
            }
            "-i" => {
                if i + 1 < args.len() {
                    input_file = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Missing value for -i");
                    std::process::exit(1);
                }
            }
            "-c" => {
                if i + 1 < args.len() {
                    color_scheme = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Missing value for -c");
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let stats = if let Some(input_file) = input_file {
        let mut file = File::open(input_file).expect("Failed to open input file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read input file");
        serde_json::from_str(&contents).expect("Failed to parse input file")
    } else if let Some(username) = username {
        // Fetch stats from GitHub API (placeholder)
        vec![]
    } else {
        eprintln!("Either -u or -i must be provided");
        std::process::exit(1);
    };

    let colors = color_scheme
        .as_ref()
        .and_then(|scheme| COLOR_SCHEMES.iter().find(|&&(name, _)| name == scheme))
        .map(|&(_, colors)| colors.to_vec());

    let chart = Chart::new(stats, colors);
    let svg = chart.render("svg").expect("Failed to render chart");

    if output_file == "-" {
        io::stdout().write_all(svg.as_bytes()).expect("Failed to write to stdout");
    } else {
        let path = Path::new(output_file);
        let mut file = File::create(&path).expect("Failed to create output file");
        file.write_all(svg.as_bytes()).expect("Failed to write to output file");
    }
}
