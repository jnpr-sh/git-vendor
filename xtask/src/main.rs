use clap::{CommandFactory, Parser, ValueEnum};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Development tasks for git-vendor")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Generate man pages for all CLI tools
    GenMan {
        /// Output directory for man pages (will be created as man1/ subdirectory)
        #[arg(short, long, default_value = "target/debug/man")]
        output: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenMan { output } => {
            if let Err(e) = generate_man_pages(&output) {
                eprintln!("Error generating man pages: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn generate_man_pages(output_dir: &PathBuf) -> std::io::Result<()> {
    let man1_dir = output_dir.join("man1");
    fs::create_dir_all(&man1_dir)?;

    println!("Generating man pages to: {}", man1_dir.display());

    generate_git_filter_tree_man(&man1_dir)?;
    generate_git_set_attr_man(&man1_dir)?;

    println!("✓ Man pages generated successfully!");
    println!("\nView with: MANPATH=target/debug/man man git-filter-tree");
    Ok(())
}

fn generate_git_filter_tree_man(output_dir: &PathBuf) -> std::io::Result<()> {
    #[derive(Parser)]
    #[command(name = "git-filter-tree")]
    #[command(author, version, about = "Filter Git tree entries by gitattributes-style patterns", long_about = None)]
    struct Cli {
        /// Tree-ish reference (commit, branch, tag, or tree SHA)
        treeish: String,

        /// Gitattributes-style patterns to filter tree entries
        #[arg(required = true)]
        patterns: Vec<String>,

        /// Output format
        #[arg(short, long, value_enum, default_value = "tree-sha")]
        format: OutputFormat,
    }

    #[derive(Clone, Copy, ValueEnum)]
    enum OutputFormat {
        /// Output only the tree SHA
        TreeSha,
        /// Output tree entries (name and type)
        Entries,
        /// Output detailed tree information
        Detailed,
    }

    let cmd = Cli::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer)?;

    let man_path = output_dir.join("git-filter-tree.1");
    fs::write(&man_path, buffer)?;

    println!("  → git-filter-tree.1");
    Ok(())
}

fn generate_git_set_attr_man(output_dir: &PathBuf) -> std::io::Result<()> {
    #[derive(Parser)]
    #[command(name = "git-set-attr")]
    #[command(author, version, about = "Set gitattributes via patterns and key-value pairs", long_about = None)]
    struct Cli {
        /// Gitattributes-style pattern (e.g. "*.txt", "path/to/*.bin")
        pattern: String,

        /// Attributes to set (e.g. "diff", "-text", "filter=lfs")
        #[arg(required = true)]
        attributes: Vec<String>,

        /// Path to the .gitattributes file to modify
        #[arg(short, long)]
        file: Option<PathBuf>,
    }

    let cmd = Cli::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer)?;

    let man_path = output_dir.join("git-set-attr.1");
    fs::write(&man_path, buffer)?;

    println!("  → git-set-attr.1");
    Ok(())
}
