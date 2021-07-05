use std::{fs::{File, OpenOptions}, io::{prelude::*, BufReader, BufWriter}, path::Path, str::FromStr};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn build_hankaku_font() -> Result<()> {
    
    let input_path = Path::new("font/hankaku.txt");

    println!("cargo:rerun-if-changed={}", input_path.display());

    let input = File::open(input_path)?;
    let input = BufReader::new(input);
    
    let output = OpenOptions::new().write(true).truncate(true).create(true).open("src/hankaku.rs").unwrap();
    let mut output = BufWriter::new(output);

    writeln!(
        &mut output,
        "pub const HANKAKU_FONT: [[u8; 16]; 256] = ["
    )?;

    let mut reading_cnt = 0;

    for line in input.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("0x") {
            writeln!(&mut output, "    //{}", line)?;
            writeln!(&mut output, "    [")?;
            reading_cnt = 0;
            continue;
        }

        reading_cnt += 1;

        let mut binary_line = String::from_str("        0b").unwrap();
        for c in line.chars() {
            match c {
                '@' => binary_line.push('1'),
                '.' => binary_line.push('0'),
                _ => binary_line.push(' '),
            }
        }
        writeln!(&mut output, "{},", binary_line);

        if reading_cnt == 16 {
            reading_cnt = 0;
            writeln!(&mut output, "    ],")?;
        }
    }
    writeln!(&mut output, "];")?;

    Ok(())
}

fn main() -> Result<()> {
    build_hankaku_font()?;
    Ok(())
}