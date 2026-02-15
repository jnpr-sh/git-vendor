mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use git_vendor::{Vendor, VendorMergeOpts};
use git2 as git;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Open the repository in current directory
    let repo = git::Repository::open(".")?;

    match cli.command {
        Commands::Track {
            pattern,
            url,
            branch,
            name,
        } => {
            repo.track_pattern(&pattern, &url, branch.as_deref(), name.as_deref())?;
            println!("Tracked pattern: {}", pattern);
            if let Some(ref n) = name {
                println!("  name: {}", n);
            }
            println!("  url: {}", url);
            if let Some(ref b) = branch {
                println!("  branch: {}", b);
            }
        }

        Commands::Untrack { pattern } => {
            repo.untrack_pattern(&pattern)?;
            println!("Untracked pattern: {}", pattern);
        }

        Commands::Status { pattern } => {
            repo.vendor_status(pattern.as_deref())?;
        }

        Commands::Fetch { pattern } => {
            repo.vendor_fetch(pattern.as_deref(), None)?;
        }

        Commands::Merge {
            pattern,
            no_commit,
            squash,
            message,
        } => {
            let opts = VendorMergeOpts {
                no_commit,
                squash,
                message,
            };
            repo.vendor_merge(pattern.as_deref(), &opts, None)?;
        }
    }

    Ok(())
}
