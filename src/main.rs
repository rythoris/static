mod cli;
mod commands;
mod utils;

use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    let mut ctx = tera::Context::new();
    let mut te = tera::Tera::default();

    for template in cli.g.templates {
        te.add_template_file(&template, None).context(format!(
            "could not include template: {}",
            template.display()
        ))?;
    }

    for include in cli.g.includes {
        let (name, p) = include;
        let name = name.unwrap_or(utils::include_name_from_filename(&p)?);
        ctx.insert(
            name,
            &utils::parse_include_file(&p)
                .context(format!("could not parse include file: {}", p.display()))?,
        );
    }

    cli.command.run(&mut te, &mut ctx)?;
    Ok(())
}
