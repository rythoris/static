use crate::cli::{Commands, GlobalArgs, ListCommandArgs, SingleCommandArgs};
use crate::utils::{self, summerize, PageData};

use std::io::Write;
use std::path::Path;
use std::{fs, io};

use anyhow::{Context, Result};
use serde::Serialize;
use tera::Tera;

impl Commands {
    pub fn run(self, g: GlobalArgs, te: &mut Tera, ctx: &mut tera::Context) -> Result<()> {
        match self {
            Commands::Single(args) => args.run(g, te, ctx),
            Commands::List(args) => args.run(g, te, ctx),
        }
    }
}

impl SingleCommandArgs {
    pub fn run(self, g: GlobalArgs, te: &mut Tera, ctx: &mut tera::Context) -> Result<()> {
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
            &g,
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
                let mut output_file = fs::File::create(&output).context(format!(
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
    pub fn run(self, g: GlobalArgs, te: &mut Tera, ctx: &mut tera::Context) -> Result<()> {
        // This shouldn't fail right? RIGHT???
        let template_name = self
            .template_file
            .file_name()
            .map(|x| x.to_str().unwrap())
            .unwrap();

        ctx.insert("page_type", "list");
        if let Some(output) = &self.output {
            ctx.insert("output_name", output.to_str().unwrap());
        }

        if let Some(content_file) = self.content {
            ctx.extend(tera::Context::from_serialize(load_markdown_page(
                &g,
                &content_file,
            )?)?);
        }

        let mut pages = Vec::with_capacity(self.pages.len());
        for page in self.pages {
            pages.push(load_markdown_page(&g, &page)?);
        }
        ctx.insert("pages", &pages);

        te.add_template_file(&self.template_file, Some(template_name))
            .context(format!(
                "could not add base template file: {}",
                self.template_file.display()
            ))?;

        let content = te.render(template_name, &ctx)?;
        match self.output {
            Some(output) => {
                let mut output_file = fs::File::create(&output).context(format!(
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

#[inline]
fn map_insert<T, S>(m: &mut PageData, key: S, val: &T)
where
    T: Serialize + ?Sized,
    S: Into<String>,
{
    m.insert(key.into(), serde_json::to_value(val).unwrap());
}

#[inline]
fn to_html(content: &str) -> Result<String> {
    let mut opts = markdown::Options::gfm();
    opts.compile.gfm_footnote_clobber_prefix = Some(String::new());
    opts.compile.gfm_footnote_label = Some(String::new());
    opts.compile.gfm_footnote_label_tag_name = Some(String::from("hr"));
    opts.compile.allow_dangerous_protocol = true;
    opts.compile.allow_dangerous_html = true;
    opts.compile.gfm_tagfilter = true;

    markdown::to_html_with_options(content, &opts).map_err(|e| anyhow::format_err!("{}", e))
}

fn load_markdown_page(g: &GlobalArgs, file: &Path) -> Result<PageData> {
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

    let summary = summerize(file_content, g.max_summary_words);
    map_insert(&mut page_data, "summary", &to_html(&summary)?);

    map_insert(&mut page_data, "body", &to_html(file_content)?);

    Ok(page_data)
}
