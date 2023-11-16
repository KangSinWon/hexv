use clap::Parser;
use std::fs;

/// Simple hex viewer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// file name
    file: Option<String>,
}

fn run(f: &str) {
    println!("\t   00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F | Decoded Text");

    let mut index = 0;
    let interval = 16;

    let mut line = 0;
    loop {
        if index > f.len() {
            break;
        }

        let s: &str;
        if index + interval > f.len() {
            s = &f[index..];
        } else {
            s = &f[index..(index + interval)];
        }

        let chars: Vec<char> = s.chars().collect();
        let mut hex_str = String::from("");
        for t in chars {
            hex_str += &format!("{:02x} ", t as i32);
        }

        if hex_str.len() < interval * 3 {

        }
        hex_str += &format!(" ").repeat((interval * 3) - hex_str.len());
        
        println!("0x{:08} {hex_str}| {}", line, s.replace("\r", "\\").replace("\n", "\\"));

        index += interval;
        line += 10;
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(file_name) = cli.file.as_deref() {
        let f = fs::read_to_string(file_name).expect("Should have been able to read the file");

        run(f.as_str());
    } else {
        println!("Please enter the file");
        std::process::exit(0);
    }
}
