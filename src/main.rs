mod cli;

use std::{fs, io, path::PathBuf};

use audio_fingerprint::{analyze_song, recognize_song};
use clap::Parser;

use crate::cli::Cli;

fn main() {
    let cli = Cli::parse();
    // Configure a simple logger

    env_logger::Builder::new()
        .filter_level(cli.verbosity.into())
        .init();

    match cli.command {
        cli::Commands::Analyze(args) => {
            log::info!(
                "Analyzing {} and committing fingerprint to database",
                args.path_to_song
            );
            analyze_song(&args.path_to_song)
        }
        cli::Commands::Recognize(args) => {
            log::info!("Attempting to recognize {}", args.path_to_song);
            match recognize_song(&args.path_to_song) {
                Some((song_metadata, match_result)) => {
                    println!("Match found:");
                    println!("Song ID: {}", song_metadata.song_id);
                    println!("Title: {}", song_metadata.title);
                    println!("Confidence: {}", match_result.confidence);
                }
                None => todo!(),
            }
        }
        cli::Commands::AnalyzeDirectory(args) => {
            log::info!("Analyzing all .wav files in {:?}", args.path_to_directory);

            let file_paths = get_file_paths_from_directory(&args.path_to_directory);
            match file_paths {
                Ok(file_paths) => {
                    for fp in file_paths.iter() {
                        analyze_song(fp);
                    }
                }
                Err(_) => todo!(),
            }
        }
    }
}

fn get_file_paths_from_directory(path_to_directory: &PathBuf) -> Result<Vec<String>, io::Error> {
    // Recursively find all .wav files in directory
    let mut file_paths = Vec::<String>::new();

    for entry in fs::read_dir(path_to_directory)? {
        let file = entry?;
        let path = file.path();

        if path.is_dir() {
            // file_paths.extend(get_file_paths_from_directory(path)?);
            continue;
        } else {
            let path_as_string = path.to_str();
            match path_as_string {
                Some(string) => file_paths.push(String::from(string)),
                None => {
                    continue;
                }
            }
        }
    }

    Ok(file_paths)
}
