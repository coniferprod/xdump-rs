use std::env;
use std::fs;
use std::io::Read;

struct DumpOptions {
    bytes_per_line: usize,

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    if let Some(buffer) = read_file(input_file) {
        dump(&buffer, &DumpOptions { bytes_per_line: 16 });
    }
}

fn dump(data: &[u8], options: &DumpOptions) {
    let line_data = data.chunks(options.bytes_per_line);
    println!("Number of {}-byte chunks = {}", 
            options.bytes_per_line, line_data.len());
    let mut offset = 0;
    for ld in line_data {
        //println!("chunk, {} bytes", ld.len());
        let line = make_line(ld, offset, options);
        println!("{}", line);
        offset += options.bytes_per_line;
    } 
}

fn make_line(data: &[u8], offset: usize, options: &DumpOptions) -> String {
    let mut line = String::new();

    line.push_str(&format!("{:06}: ", offset));

    let mut chars = " ".to_string();
    let mut bytes_done = 0;
    for b in data {
        line.push_str(&format!("{:02x} ", b));

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

fn read_file(name: &String) -> Option<Vec<u8>> {
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
