pub mod commands;

use clap::Parser;
use colored::Colorize;
use commands::{Cli, SubCommands};
use regex::Regex;
use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

const GITHUB_API_URL: &str = "https://api.github.com/repos/github/gitignore/contents";
const RAW_CONTENT_BASE_URL: &str = "https://raw.githubusercontent.com/github/gitignore/main";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let client = Client::builder()
        .user_agent("gitignore-generator")
        .timeout(Duration::from_secs(30))
        .build()?;

    match cli.extra_command {
        Some(SubCommands::List) => {
            let templates = get_templates(&client).await?;
            for template in templates {
                println!("{template}");
            }
        }
        Some(SubCommands::Search { target_search }) => {
            let templates = get_templates(&client).await?;

            let matched_languages: Vec<&String> = 
                templates
                    .iter()
                    .filter(|template| {
                        template.contains(&target_search) || 
                        template.eq_ignore_ascii_case(&target_search)
                    })
                    .collect();

            for match_language in matched_languages {
                println!("{}", match_language.green());
            }
        }
        Some(SubCommands::RegexSearch { regex }) => {
            let templates = get_templates(&client).await?;
            
            println!("Current using regex is {}", regex.yellow().italic());
            
            let regex = Regex::new(&regex).unwrap();

            let matched_languages: Vec<&String> = 
                templates
                .iter()
                    .filter(|template| {
                        regex.is_match(&template)
                    })
                    .collect();

            for match_language in matched_languages {
                println!("{}", match_language.green());
            }
        }
        Some(SubCommands::Generate { template_language, output }) => {
            generate_gitignore(&client, &template_language, &output).await?;
            println!(
                "Successfully generated {} with templates: {:?}",
                output, template_language
            );
        },
        None => {

        }
    }
    Ok(())
}

async fn get_templates(client: &Client) -> Result<Vec<String>, Box<dyn Error>> {
    let response = client
        .get(GITHUB_API_URL)
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch templates: {}", response.status()).into());
    }

    let contents: Vec<GitHubContent> = response.json().await?;

    let languages: Vec<String> = contents
        .into_iter()
        .filter_map(|content| {
            if content.name.ends_with(".gitignore") {
                Some(content.name.replace(".gitignore", ""))
            } else {
                None
            }
        }).collect();

    Ok(languages)
}

async fn generate_gitignore(
    client: &Client,
    template: &str,
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    output.push_str("# Generated gitignore file\n");
    output.push_str("# Created with ignore-gen\n\n");

    let template_name = if template.ends_with(".gitignore") {
        template.to_string()
    } else {
        format!("{}.gitignore", template)
    };

    let url = format!("{}/{}", RAW_CONTENT_BASE_URL, template_name);
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        eprintln!("Warning: Template '{}' not found", template);
    }

    let content = response.text().await?;
    output.push_str(&format!("# ===== {} =====\n", template));
    output.push_str(&content);
    output.push_str("\n\n");
    output.push_str("# End of Generated File with ignore-gen\n\n");

    let path = Path::new(output_file);
    let mut file = File::create(path)?;
    file.write_all(output.as_bytes())?;

    Ok(())
}

#[derive(serde::Deserialize)]
struct GitHubContent {
    name: String,
}
