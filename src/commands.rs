use clap::Args;

#[derive(clap::Parser)]
pub struct Cli {
    #[clap(flatten)]
    pub generate_args: GenerateArgs,

    #[command(subcommand)]
    pub extra_command: Option<SubCommands>,
}

#[derive(Args)]
pub struct GenerateArgs {
    /// Template names (e.g., Rust, Golang, Python, Node)
    #[arg(short = 't', long = "template")]
    pub template_languages: Vec<String>,

    /// Output file
    #[arg(short, long, default_value = ".gitignore")]
    pub output: String,
}

#[derive(clap::Subcommand)]
pub enum SubCommands {
    List,
    Search {
        #[arg(short = 't', long = "template", required = true)]
        target_search: String,
    }
}
