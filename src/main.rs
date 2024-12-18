mod cli;
mod commands;
mod utils;

use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    let mut ctx = tera::Context::new();
    let mut te = tera::Tera::default();

    for template in &cli.g.templates {
        te.add_template_file(
            &template,
            Some(template.file_name().unwrap().to_str().unwrap()),
        )
        .context(format!(
            "could not include template: {}",
            template.display()
        ))?;
    }

    for include in &cli.g.includes {
        let (name, p) = include;
        let fname = utils::include_name_from_filename(&p)?;
        let name = name.as_ref().unwrap_or(&fname);
        ctx.insert(
            name,
            &utils::parse_include_file(&p)
                .context(format!("could not parse include file: {}", p.display()))?,
        );
    }

    cli.command.run(cli.g, &mut te, &mut ctx)?;
    Ok(())
}
