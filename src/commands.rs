#[derive(clap::Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub extra_command: Option<SubCommands>,
}

#[derive(clap::Subcommand)]
pub enum SubCommands {
    /// Generate gitignore from required language
    Generate {
        /// Template names (e.g., C++, Rust, Golang, Python, Node)
        #[arg(short = 't', long = "template")]
        template_language: String,

        /// Output file
        #[arg(short, long, default_value = ".gitignore")]
        output: String,
    },
    /// List all .gitignore templates available on github
    List,
    /// Search from github, case insensitive
    Search {
        target_search: String,
    },
    /// Search from github, use regex to match languages.
    /// !!!Attention!!!, use single quotes in shell, like: '^C.+$' and matches with C prefix
    RegexSearch {
        regex: String,
    }
}
