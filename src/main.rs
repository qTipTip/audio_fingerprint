mod cli;

use std::{fs, io};

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
        cli::Commands::Analyze { path_to_song } => {
            log::info!(
                "Analyzing {} and committing fingerprint to database",
                path_to_song
            );
            analyze_song(&path_to_song)
        }
        cli::Commands::Recognize {
            path_to_song_snippet,
        } => {
            log::info!("Attempting to recognize {}", path_to_song_snippet);
            match recognize_song(&path_to_song_snippet) {
                Some(song) => {
                    log::info!(
                        "Found a match! song_id: {} title: {}",
                        song.song_id,
                        song.title
                    );
                }
                None => todo!(),
            }
        }
        cli::Commands::AnalyzeDirectory { path_to_directory } => {
            log::info!("Analyzing all .wav files in {}", path_to_directory);

            let file_paths = get_file_paths_from_directory(&path_to_directory);
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

fn get_file_paths_from_directory(path_to_directory: &str) -> Result<Vec<String>, io::Error> {
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
