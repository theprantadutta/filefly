use clap::Parser;

use crate::logger::LogLevel;

// Define command-line argument structure using the clap parser
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

    #[clap(name = "upgrade", about = "Upgrade Filefly to Latest Version")]
    Upgrade(UpgradeCommand),
}

// Define subcommand structures for each operation
#[derive(Parser, Debug)]
#[clap(name = "subcommand")]
pub struct CopyCommand {
    /// Source directory
    #[clap(short, long)]
    pub source: String,

    /// Destination directory
    #[clap(short, long)]
    pub destination: String,

    /// Disable logging
    #[clap(long, default_value_t = false)] // Optional, defaults to false
    pub no_log: bool,

    /// Log level (debug, info, success, warning, error)
    #[clap(long, value_parser, default_value = "info")]
    pub log_level: LogLevel,
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

    /// Disable logging
    #[clap(long, default_value_t = false)] // Optional, defaults to false
    pub no_log: bool,

    /// Log level (debug, info, success, warning, error)
    #[clap(long, value_parser, default_value = "info")]
    pub log_level: LogLevel,
}

#[derive(Parser, Debug)]
#[clap(name = "subcommand")]
pub struct DeleteCommand {
    /// Folder to delete
    #[clap(short, long)]
    pub folder: String,

    /// Disable logging
    #[clap(long, default_value_t = false)] // Optional, defaults to false
    pub no_log: bool,

    /// Log level (debug, info, success, warning, error)
    #[clap(long, value_parser, default_value = "info")]
    pub log_level: LogLevel,
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

    /// Do not delete files
    #[clap(long)]
    pub no_delete: bool,

    /// Disable logging
    #[clap(long, default_value_t = false)] // Optional, defaults to false
    pub no_log: bool,

    /// Log level (debug, info, success, warning, error)
    #[clap(long, value_parser, default_value = "info")]
    pub log_level: LogLevel,
}

#[derive(Parser, Debug)]
#[clap(name = "subcommand")]
pub struct UpgradeCommand {}
