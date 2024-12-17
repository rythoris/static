use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use anyhow::Result;

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
    #[arg(short, long, value_parser = include_file_parser)]
    pub includes: Vec<(Option<String>, PathBuf)>,

    #[arg(short, long)]
    pub templates: Vec<PathBuf>,
}

#[derive(Args, Debug)]
pub struct SingleCommandArgs {
    pub base_template: PathBuf,
    pub content: PathBuf,

    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct ListCommandArgs {
    pub base_template: PathBuf,
    pub files: Vec<PathBuf>,

    #[arg(short, long)]
    pub content: Option<PathBuf>,

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
