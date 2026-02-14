use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "git-set-attr")]
#[command(author, version, about = "Set gitattributes via patterns and key-value pairs", long_about = None)]
pub struct Cli {
    /// Gitattributes-style pattern (e.g. "*.txt", "path/to/*.bin")
    pub pattern: String,

    /// Attributes to set (e.g. "diff", "-text", "filter=lfs")
    #[arg(required = true)]
    pub attributes: Vec<String>,

    /// Path to the .gitattributes file to modify
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}
