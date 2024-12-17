use crate::cli::{Commands, ListCommandArgs, SingleCommandArgs};
use crate::utils;

use std::io::Write;
use std::{fs, io};

use anyhow::{Context, Result};
use tera::Tera;

impl Commands {
    pub fn run(self, te: &mut Tera, ctx: &mut tera::Context) -> Result<()> {
        match self {
            Commands::Single(args) => args.run(te, ctx),
            Commands::List(args) => args.run(te, ctx),
        }
    }
}

impl SingleCommandArgs {
    pub fn run(self, te: &mut Tera, ctx: &mut tera::Context) -> Result<()> {
        // These shouldn't fail right? RIGHT???
        let template_name = self
            .template_file
            .file_name()
            .map(|x| x.to_str().unwrap())
            .unwrap();
        let input_name = self
            .input_file
            .file_name()
            .map(|x| x.to_str().unwrap())
            .unwrap();

        let input_file_content = fs::read_to_string(&self.input_file).context(format!(
            "could not read input file: {}",
            self.input_file.display()
        ))?;

        let (frontmatter, input_file_content) =
            utils::frontmatter(&input_file_content).context("could not parse frontmatter")?;

        if let Some(frontmatter) = frontmatter {
            ctx.insert("frontmatter", &frontmatter);
        }

        ctx.insert("file_name", input_name);
        ctx.insert(
            "body",
            &markdown::to_html_with_options(input_file_content, &markdown::Options::gfm())
                .map_err(|e| anyhow::format_err!("{}", e))?,
        );

        te.add_template_file(&self.template_file, Some(template_name))
            .context(format!(
                "could not add base template file: {}",
                self.template_file.display()
            ))?;

        let content = te.render(template_name, &ctx)?;
        match self.output {
            Some(output) => {
                let mut output_file = fs::File::open(&output).context(format!(
                    "could not open the output file for writing: {}",
                    output.display()
                ))?;
                write!(output_file, "{}", content).context(format!(
                    "could not write to output file: {}",
                    output.display()
                ))?;
            }
            None => {
                write!(io::stdout().lock(), "{}", content).context("could not write to stdout")?;
            }
        }

        Ok(())
    }
}

impl ListCommandArgs {
    pub fn run(self, te: &mut Tera, ctx: &mut tera::Context) -> Result<()> {
        unimplemented!()
    }
}
