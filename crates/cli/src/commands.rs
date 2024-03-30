#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    pub glob: String,
    pub command: String,
    pub args: Vec<String>,
}
