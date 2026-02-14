mod cli;

use clap::Parser;
use cli::{Cli, OutputFormat};
use git_filter_tree::FilterTree;
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

    // Resolve the tree-ish to a tree
    let obj = repo.revparse_single(&cli.treeish)?;
    let tree = obj.peel_to_tree()?;

    // Convert patterns to string slices
    let patterns: Vec<&str> = cli.patterns.iter().map(|s| s.as_str()).collect();

    // Filter the tree by patterns
    let filtered_tree = repo.filter_by_patterns(&tree, &patterns)?;

    // Output based on format
    match cli.format {
        OutputFormat::TreeSha => {
            println!("{}", filtered_tree.id());
        }
        OutputFormat::Entries => {
            for entry in filtered_tree.iter() {
                let name = entry.name().unwrap_or("<invalid-utf8>");
                let kind = match entry.kind() {
                    Some(git::ObjectType::Blob) => "blob",
                    Some(git::ObjectType::Tree) => "tree",
                    Some(git::ObjectType::Commit) => "commit",
                    _ => "unknown",
                };
                println!("{}\t{}", kind, name);
            }
        }
        OutputFormat::Detailed => {
            println!("Tree: {}", filtered_tree.id());
            println!("Entries: {}", filtered_tree.len());
            println!();
            for entry in filtered_tree.iter() {
                let name = entry.name().unwrap_or("<invalid-utf8>");
                let kind = match entry.kind() {
                    Some(git::ObjectType::Blob) => "blob",
                    Some(git::ObjectType::Tree) => "tree",
                    Some(git::ObjectType::Commit) => "commit",
                    _ => "unknown",
                };
                let mode = entry.filemode();
                let id = entry.id();
                println!("{:06o} {} {}\t{}", mode, kind, id, name);
            }
        }
    }

    Ok(())
}
