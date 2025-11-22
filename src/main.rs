use md5::Context;
use sha2::{Digest as ShaDigest, Sha256};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <expected_hash> <file>", args[0]);
        std::process::exit(1);
    }

    let expected: String = args[1].to_lowercase();
    let filename: &String = &args[2];

    let file: File = File::open(filename)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut buffer: [u8; 8192] = [0u8; 8192];

    let result: String = match expected.len() {
        32 => {
            // MD5
            let mut context: Context = Context::new();
            loop {
                let n: usize = reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                context.consume(&buffer[..n]);
            }
            let digest: md5::Digest = context.finalize();
            format!("{:x}", digest)
        }
        64 => {
            // SHA256
            let mut hasher = Sha256::new();
            loop {
                let n: usize = reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            format!("{:x}", hasher.finalize())
        }
        _ => {
            eprintln!("Hash length not recognized. Use MD5 (32 chars) or SHA256 (64 chars).");
            std::process::exit(1);
        }
    };

    if result == expected {
        println!("Hash OK");
    } else {
        println!("Hash MISMATCH!");
        println!("Expected: {}", expected);
        println!("Found:    {}", result);
    }

    Ok(())
}
