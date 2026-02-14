use clap::Parser;

#[derive(Parser)]
#[command(name = "git-filter-tree")]
#[command(author, version, about = "Filter Git tree entries by gitattributes-style patterns", long_about = None)]
pub struct Cli {
    /// Tree-ish reference (commit, branch, tag, or tree SHA)
    pub treeish: String,

    /// Gitattributes-style patterns to filter tree entries
    #[arg(required = true)]
    pub patterns: Vec<String>,

    /// Output format
    #[arg(short, long, value_enum, default_value = "tree-sha")]
    pub format: OutputFormat,
}

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    /// Output only the tree SHA
    TreeSha,
    /// Output tree entries (name and type)
    Entries,
    /// Output detailed tree information
    Detailed,
}
