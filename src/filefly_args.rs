use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "command")]
pub enum FileFlyArgs {
    #[clap(short_flag = 'c', alias = "cp", name = "copy", about = "Copy files from source to destination")]
    Copy(CopyCommand),

    #[clap(short_flag = 'd', alias = "del", name = "delete", about = "Delete a folder")]
    Delete(DeleteCommand),
}

#[derive(Parser, Debug)]
#[clap(name = "subcommand")]
pub struct CopyCommand {
    /// Source directory
    #[clap(short, long)]
    pub source: String,

    /// Destination directory
    #[clap(short, long)]
    pub destination: String,
}

#[derive(Parser, Debug)]
#[clap(name = "subcommand")]
pub struct DeleteCommand {
    /// Folder to delete
    #[clap(short, long)]
    pub folder: String,
}