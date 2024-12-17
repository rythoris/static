use std::fs;
use std::path::Path;
use std::collections::BTreeMap;

use anyhow::{Result, Context};

pub fn include_name_from_filename(p: &Path) -> Result<String> {
    let valid_filename_re = regex::Regex::new(r"[A-z][A-z0-9-_]").unwrap();
    let p = p.with_extension("");
    let ps = p.file_name().unwrap().to_str().unwrap();

    if valid_filename_re.is_match(ps) {
        Ok(ps.to_string())
    } else {
        anyhow::bail!("invalid include filename, you can implicitly define the name for the include or use a valid filename.")
    }
}

pub fn parse_include_file(p: &Path) -> Result<BTreeMap<String, serde_json::Value>> {
    let typ = p.extension().map(|x| x.to_str().unwrap());
    let read_include_file =
        || fs::read_to_string(p).context(format!("could not read include file: {}", p.display()));

    match typ {
        Some(ext) => Ok(match ext {
            "json" => serde_json::from_str(&read_include_file()?)?,
            "yaml" | "yml" => serde_yaml::from_str(&read_include_file()?)?,
            "toml" => toml::from_str(&read_include_file()?)?,
            _ => anyhow::bail!("invalid file extention: {}", ext),
        }),
        None => anyhow::bail!("file name does not have an extention."),
    }
}
