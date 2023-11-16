use clap::Parser;
use colored::*;
use std::fs::File;
use std::io::Read;

/// Simple hex viewer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// file name
    file: Option<String>,
}

struct Hexv {
    buf: Vec<u8>,
    idx: usize,
    interval: usize,
    line: usize,
}

fn run(mut hexv: Hexv) {
    loop {
        if hexv.idx >= hexv.buf.len() {
            break;
        }

        let buf_slice: Vec<u8>;
        if hexv.idx + hexv.interval > hexv.buf.len() {
            buf_slice = hexv.buf[hexv.idx..].to_vec();
        } else {
            buf_slice = hexv.buf[hexv.idx..(hexv.idx + hexv.interval)].to_vec();
        }

        let mut hex_len = 0;
        let mut hex_str = String::from("");
        let mut dec_str = String::from("");
        for &t in &buf_slice {
            if t == 0 {
                dec_str += &"0".black().to_string();
            } else if t > 9 && t < 14 {
                dec_str += &"\\".red().to_string();
            } else if t < 32 || t > 126 {
                dec_str += &"x".yellow().to_string();
            } else {
                dec_str += &(t as char).to_string().blue().to_string();
            }

            hex_str += &format!("{:02x} ", t as i32).green().to_string();
            hex_len += 3;
        }
        
        if (hexv.interval * 3) - hex_len > 0 {
            hex_str += &format!(" ").repeat((hexv.interval * 3) - hex_len);
        }

        println!(
            "0x{:08x} {}| {}",
            hexv.line,
            hex_str,
            dec_str
        );

        hexv.idx += hexv.interval;
        hexv.line += hexv.interval;
    }
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    if let Some(file_name) = cli.file.as_deref() {
        println!("hexv: {}", file_name);

        let mut f = File::open(file_name)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;

        println!("\t   00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F | Decoded Text");

        let hexv = Hexv {
            buf,
            idx: 0,
            interval: 16,
            line: 0,
        };

        run(hexv);
    } else {
        println!("Please enter the file");
    }

    Ok(())
}