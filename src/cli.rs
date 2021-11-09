use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Items {
    /// Add entry.
    Add {
        #[structopt()]
        task: String,
    },

    /// Remove an entry.
    Done {
        #[structopt()]
        position: usize,
    },

    /// List entries.
    List,
}

#[derive(Debug, StructOpt)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Items,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal: Option<PathBuf>,
}
