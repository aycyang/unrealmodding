use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::process::exit;
use std::time::SystemTime;

use clap::{Parser, Subcommand};

use upak;

/// Command line tool for working with Unreal Engine .pak files
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// What to do
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds files to myapp
    Check {
        /// The .pak file to check
        pakfile: String
    },
    CheckHeader {
        /// The .pak file to check
        pakfile: String
    },
    Extract {
        /// The .pak file to extract
        pakfile: String,
        /// The directory to extract to
        outdir: Option<String>
    },
    Create {
        /// The .pak file to create
        pakfile: String,
        /// The directory to create the file from
        indir: String
    },
}

fn main() {
    let args = Args::parse();

    let start = SystemTime::now();

    match args.commands {
        Commands::CheckHeader { pakfile } => {
            let file = open_file(Path::new(&pakfile));
            let pak = upak::PakFile::new(&file);
            check_header(pak);
        },
        _ => {
            eprintln!("Not implemented yet");
        }
    }


    println!(
        "upakcli took {:?} seconds...",
        start.elapsed().unwrap().as_secs_f32()
    )
}

fn open_file(path: &Path) -> File {
    match OpenOptions::new().read(true).open(&path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Could not find/open file");
            exit(1);
        }
    }
}

fn check_header(mut pak_file: upak::PakFile) -> upak::PakFile {
    match pak_file.load_records() {
        Ok(_) => println!("Header is ok"),
        Err(e) => {
            eprintln!("Error reading header: {:?}", e);
            exit(1);
        }
    }
    println!("Found {:?} records", pak_file.records.len());
    return pak_file;
}