use crate::cli::{Commands, ListCommandArgs, SingleCommandArgs};
use crate::utils::{self, PageData};

use std::io::Write;
use std::path::Path;
use std::{fs, io};

use anyhow::{Context, Result};
use serde::Serialize;
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
        // This shouldn't fail right? RIGHT???
        let template_name = self
            .template_file
            .file_name()
            .map(|x| x.to_str().unwrap())
            .unwrap();

        ctx.insert("page_type", "single");
        if let Some(output) = &self.output {
            ctx.insert("output_name", output.to_str().unwrap());
        }

        ctx.extend(tera::Context::from_serialize(load_markdown_page(
            &self.input_file,
        )?)?);

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

pub fn map_insert<T, S>(m: &mut PageData, key: S, val: &T)
where
    T: Serialize + ?Sized,
    S: Into<String>,
{
    m.insert(key.into(), serde_json::to_value(val).unwrap());
}

fn load_markdown_page(file: &Path) -> Result<PageData> {
    let mut page_data = PageData::new();
    let file_name = file.file_name().map(|x| x.to_str().unwrap()).unwrap();

    let file_content =
        fs::read_to_string(&file).context(format!("could not read file: {}", file.display()))?;

    let (frontmatter, file_content) =
        utils::frontmatter(&file_content).context("could not parse frontmatter text")?;

    if let Some(frontmatter) = frontmatter {
        map_insert(&mut page_data, "frontmatter", &frontmatter);
    }

    map_insert(&mut page_data, "file_name", file_name);
    map_insert(
        &mut page_data,
        "body",
        &markdown::to_html_with_options(file_content, &markdown::Options::gfm())
            .map_err(|e| anyhow::format_err!("{}", e))?,
    );

    Ok(page_data)
}
