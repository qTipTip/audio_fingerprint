use clap::{Parser, Subcommand};
use clap_verbosity_flag::InfoLevel;

#[derive(Debug, Parser)]
#[command(name = "audio_fingerprint")]
#[command(about = "An audio fingerprinting and song recognizing CLI", long_about = None)]

pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<InfoLevel>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    Analyze { path_to_song: String },
    Recognize { path_to_song_snippet: String },
}
