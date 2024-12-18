use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    pub g: GlobalArgs,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug)]
pub struct GlobalArgs {
    /// var files
    #[arg(short, long, value_parser = include_file_parser)]
    pub includes: Vec<(Option<String>, PathBuf)>,

    /// global template files
    #[arg(short, long)]
    pub templates: Vec<PathBuf>,

    /// number of words in the auto-generated summary when `<!-- more -->` was not present
    #[arg(short, long, default_value = "50")]
    pub max_summary_words: usize,
}

#[derive(Args, Debug)]
pub struct SingleCommandArgs {
    /// base template
    #[arg(value_name = "TEMPLATE_FILE")]
    pub template_file: PathBuf,

    /// input markdown file
    #[arg(value_name = "INPUT_FILE")]
    pub input_file: PathBuf,

    #[arg(short, long)]
    /// output path (If not provided, the content will be printed to stdout)
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct ListCommandArgs {
    /// base template
    #[arg(value_name = "TEMPLATE_FILE")]
    pub template_file: PathBuf,

    /// pages
    #[arg(value_name = "FILE", num_args(1..))]
    pub pages: Vec<PathBuf>,

    /// template body content file (this is simular to the `single` command input file)
    #[arg(short, long, value_name = "FILE")]
    pub content: Option<PathBuf>,

    /// output path (If not provided, the content will be printed to stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Single(SingleCommandArgs),
    List(ListCommandArgs),
}

fn include_file_parser(s: &str) -> Result<(Option<String>, PathBuf)> {
    let path_regex = regex::Regex::new(r"^((?P<name>[A-z][A-z0-9_-]+)@)?(?P<path>.*)$").unwrap();

    let (name, path) = path_regex
        .captures_iter(s)
        .map(|caps| {
            let name = caps.name("name").map(|x| x.as_str().to_string());
            let path = PathBuf::from(caps.name("path").unwrap().as_str());
            (name, path)
        })
        .next()
        .unwrap();

    if !path.is_file() {
        anyhow::bail!("include file is not a regular file.");
    }

    Ok((name, path))
}
