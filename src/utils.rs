use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

pub type PageData = BTreeMap<String, serde_json::Value>;

pub fn include_name_from_filename(p: &Path) -> Result<String> {
    let valid_filename_re = regex::Regex::new(r"[A-z][A-z0-9-_]").unwrap();

    let p = p.with_extension(""); // remove file extention.
    let ps = p.file_name().unwrap().to_str().unwrap(); // get basename

    if valid_filename_re.is_match(ps) {
        Ok(ps.to_string())
    } else {
        anyhow::bail!("invalid include filename. (you can implicitly define the name for the include or use a valid filename. see --help for more information)")
    }
}

pub fn parse_include_file(p: &Path) -> Result<PageData> {
    // function for reading file. it is defined to make the code easier to read.
    let read_include_file = || fs::read_to_string(p).context(format!("could not read file"));

    match p.extension().map(|x| x.to_str().unwrap()) {
        Some(ext) => Ok(match ext {
            "json" => serde_json::from_str(&read_include_file()?)?,
            "yaml" | "yml" => serde_yaml::from_str(&read_include_file()?)?,
            "toml" => toml::from_str(&read_include_file()?)?,
            _ => anyhow::bail!("file type not supported."),
        }),
        None => anyhow::bail!(
            "file name does not have an extention. (extention is used for detecting the file-type)"
        ),
    }
}

enum FrontmatterType {
    Yaml,
    Toml,
    Json,
}
const FRONTMATTER_FORMATS: [(&str, &str, FrontmatterType); 6] = [
    ("---\n", "---\n", FrontmatterType::Yaml),
    ("---yaml\n", "---\n", FrontmatterType::Yaml),
    ("+++\n", "+++\n", FrontmatterType::Toml),
    ("---toml\n", "---\n", FrontmatterType::Toml),
    (";;;\n", ";;;\n", FrontmatterType::Json),
    ("---json\n", "---\n", FrontmatterType::Json),
];

// TODO: write tests for `frontmatter`
pub fn frontmatter<'a>(content: &'a str) -> Result<(Option<PageData>, &'a str)> {
    let fmt = FRONTMATTER_FORMATS
        .iter()
        .filter(|(start, ..)| content.starts_with(start))
        .collect::<Vec<_>>();

    match fmt.first() {
        Some((start_marker, end_marker, typ)) => {
            let start = start_marker.len();
            match &content[start..].find(end_marker) {
                Some(end) => {
                    let frontmatter_end = start + end + end_marker.len();
                    let frontmatter_data = &content[start..start + end];
                    let frontmatter = match typ {
                        FrontmatterType::Yaml => serde_yaml::from_str(frontmatter_data)?,
                        FrontmatterType::Toml => toml::from_str(frontmatter_data)?,
                        FrontmatterType::Json => serde_json::from_str(frontmatter_data)?,
                    };
                    Ok((Some(frontmatter), &content[frontmatter_end..]))
                }
                None => anyhow::bail!("frontmatter terminator not found"),
            }
        }
        None => Ok((None, content)),
    }
}
