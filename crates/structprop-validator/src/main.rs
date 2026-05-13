use std::io::{self, Read};
use std::path::PathBuf;
use std::process::ExitCode;
use std::{env, fs};

use anyhow::{Context, Result};
use serde_structprop::parse;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String]) -> Result<()> {
    match args {
        [] => {
            // No arguments — read from stdin
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .context("failed to read from stdin")?;
            validate("<stdin>", &input)
        }
        [path] if path == "--help" || path == "-h" => {
            print_usage();
            Ok(())
        }
        [path] if path == "--version" || path == "-V" => {
            println!("structprop-validator {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        paths => {
            // One or more file arguments
            let mut had_error = false;
            for path in paths {
                let p = PathBuf::from(path);
                match fs::read_to_string(&p)
                    .with_context(|| format!("failed to read '{}'", p.display()))
                {
                    Ok(input) => {
                        if let Err(e) = validate(&p.display().to_string(), &input) {
                            eprintln!("error: {e:#}");
                            had_error = true;
                        }
                    }
                    Err(e) => {
                        eprintln!("error: {e:#}");
                        had_error = true;
                    }
                }
            }
            if had_error {
                anyhow::bail!("one or more files failed validation");
            }
            Ok(())
        }
    }
}

fn validate(name: &str, input: &str) -> Result<()> {
    parse(input).with_context(|| format!("'{name}' is not valid structprop"))?;
    println!("{name}: ok");
    Ok(())
}

fn print_usage() {
    eprintln!(
        "Usage: structprop-validator [FILE]...

Validate one or more structprop files.  If no FILE is given, reads from stdin.

Exit status:
  0  all inputs are valid
  1  one or more inputs contain errors
"
    );
}
