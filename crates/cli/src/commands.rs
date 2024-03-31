#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    pub glob: String,
    pub command: String,
    pub args: Vec<String>,

    #[arg(short, long)]
    /// If set, the command will not be invoked until the first file change event is received.
    pub skip_launch_on_startup: bool,
}
