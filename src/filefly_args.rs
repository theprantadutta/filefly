use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "command")]
pub enum FileFlyArgs {
    #[clap(
        short_flag = 'c',
        alias = "cp",
        name = "copy",
        about = "Copy file/folder from source to destination"
    )]
    Copy(CopyCommand),

    #[clap(
        short_flag = 'd',
        alias = "del",
        name = "delete",
        about = "Delete a file/folder"
    )]
    Delete(DeleteCommand),

    #[clap(
        short_flag = 'C',
        alias = "cut",
        name = "replace",
        about = "Replace a file/folder with another"
    )]
    Replace(ReplaceCommand),

    #[clap(
        short_flag = 's',
        alias = "sync",
        name = "synchronize",
        about = "Synchronize a file/folder with another"
    )]
    Synchronize(SynchronizeCommand),
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
pub struct ReplaceCommand {
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

#[derive(Parser, Debug)]
#[clap(name = "subcommand")]
pub struct SynchronizeCommand {
    /// Source directory
    #[clap(short, long)]
    pub source: String,

    /// Destination directory
    #[clap(short, long)]
    pub destination: String,
}
