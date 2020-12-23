//! Handle command line arguments using the argh crate.

use argh::FromArgs;

/// Command line arguments.
#[derive(FromArgs)]
#[argh(description = "Text editor with ctrl+q to quit and ctrl+s to save.")]
pub struct CmdArgs {
    #[argh(switch, description = "display version and exit.")]
    pub version: bool,

    #[argh(positional, description = "open files at startup")]
    pub file_paths: Vec<String>,
}
