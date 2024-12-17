mod cli;
mod commands;
mod utils;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    let mut ctx = tera::Context::default();
    let mut te = tera::Tera::default();

    for template in cli.g.templates {
        te.add_template_file(template, None)?;
    }

    for include in cli.g.includes {
        let (name, p) = include;
        let name = name.unwrap_or(utils::include_name_from_filename(&p)?);

        ctx.insert(name, &utils::parse_include_file(&p)?);
    }

    cli.command.run(&mut te, &mut ctx)?;

    Ok(())
}
