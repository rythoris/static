use crate::cli::{Commands, SingleCommandArgs, ListCommandArgs};

use anyhow::Result;
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
        unimplemented!()
    }
}


impl ListCommandArgs {
    pub fn run(self, te: &mut Tera, ctx: &mut tera::Context) -> Result<()> {
        unimplemented!()
    }
}
