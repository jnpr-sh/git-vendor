mod cli;

use clap::Parser;
use cli::Cli;
use git_set_attr::SetAttr;
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

    // Convert attributes to string slices
    let attributes: Vec<&str> = cli.attributes.iter().map(|s| s.as_str()).collect();

    // Set attributes in the appropriate .gitattributes file
    repo.set_attr(&cli.pattern, &attributes, cli.file.as_deref())?;

    Ok(())
}
