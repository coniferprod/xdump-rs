use std::{
    fs,
    io::Read,
    path::PathBuf
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    /// The offset in the file where the dump should start
    start: Option<u64>,

    #[arg(short, long)]
    /// The length of the dump starting from the offset
    length: Option<u64>,

    input_file: PathBuf,
}

#[derive(Debug)]
struct DumpOptions {
    bytes_per_line: u64,
    start_offset: u64,
    dump_length: u64,
    lowercase: bool,
}

fn main() {
    let args = Args::parse();

    let metadata = fs::metadata(&args.input_file).expect("file exists");

    // TODO: Check that the start offset and/or dump length
    // are not negative or beyond the file size.
    let options = DumpOptions {
        bytes_per_line: 16,
        start_offset: if let Some(start) = args.start { start } else { 0 },
        dump_length: if let Some(length) = args.length { length } else { metadata.len() },
        lowercase: false,
    };

    println!("{:?}", options);

    if let Some(buffer) = read_file(args.input_file.to_str().expect("valid path")) {
        let start = options.start_offset as usize;
        let end = start + options.dump_length as usize;
        dump(&buffer[start .. end], &options);
    }
}

fn dump(data: &[u8], options: &DumpOptions) {
    let line_data = data.chunks(options.bytes_per_line as usize);
    let mut offset = options.start_offset;
    for ld in line_data {
        let line = make_line(ld, offset, options);
        println!("{}", line);
        offset += options.bytes_per_line;
    }
}

fn make_line(data: &[u8], offset: u64, options: &DumpOptions) -> String {
    let mut line = String::new();

    line.push_str(&format!("{:08}: ", offset));

    let mut chars = " ".to_string();
    let mut bytes_done = 0;
    for b in data {
        let bs = if options.lowercase {
            format!("{:02x} ", b)
        } else {
            format!("{:02X} ", b)
        };
        line.push_str(&bs);

        match *b {
            0x20 ..= 0x7F => chars.push(*b as char),
            _ => chars.push('.')
        }

        bytes_done += 1;
    }

    while bytes_done < options.bytes_per_line {
        line.push_str("   ");
        chars.push(' ');
        bytes_done += 1;
    }

    line.push_str(&chars);

    line
}

fn read_file(name: &str) -> Option<Vec<u8>> {
    match fs::File::open(&name) {
        Ok(mut f) => {
            let mut buffer = Vec::new();
            match f.read_to_end(&mut buffer) {
                Ok(_) => Some(buffer),
                Err(_) => None
            }
        },
        Err(_) => {
            eprintln!("Unable to open file {}", &name);
            None
        }
    }
}
