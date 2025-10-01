use std::path::PathBuf;

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

#[derive(clap::Args, Debug)]
pub(crate) struct AnalyzeArgs {
    #[arg(long, short = 'p')]
    pub path_to_song: String,
}

#[derive(clap::Args, Debug)]
pub(crate) struct AnalyzeDirectoryArgs {
    #[arg(long, short = 'p')]
    pub path_to_directory: PathBuf,
}

#[derive(clap::Args, Debug)]
pub(crate) struct RecognizeArgs {
    #[arg(long, short = 'p')]
    pub path_to_song: String,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    Analyze(AnalyzeArgs),
    AnalyzeDirectory(AnalyzeDirectoryArgs),
    Recognize(RecognizeArgs),
}
