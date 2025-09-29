mod cli;

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
            recognize_song(&path_to_song_snippet);
        }
    }
}
