extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    println!("Compress file");
    if args().len() < 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

    let mut input;
    let src_file_result = File::open(args().nth(1).unwrap());
    match src_file_result {
        Ok(src_file) => {
            input = BufReader::new(src_file);
        }
        Err(err) => {
            eprintln!("Error opening source file: {}", err);
            return;
        }
    };

    let output;
    let dest_file_result = File::create(args().nth(2).unwrap());
    match dest_file_result {
        Ok(dest_file) => output = dest_file,
        Err(err) => {
            eprintln!("Error creating destination file: {}", err);
            return;
        }
    };

    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();
    let output_result = encoder.finish();
    let output;
    match output_result {
        Ok(file) => output = file,
        Err(err) => {
            eprintln!("Error encoding: {}", err);
            return;
        }
    };
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed time: {:?}", start.elapsed());
}
