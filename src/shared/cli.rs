use clap::{command, Parser, Subcommand};

use serde::{Deserialize, Serialize};

/// Arguments to client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Command passed
    #[command(subcommand)]
    pub command: BanyanCommand,
    /// Verbosity level.
    #[arg(short = 'v', long, action = clap::ArgAction::Count, help = "Increase verbosity level (-v, -vv, -vvv)")]
    pub verbose: u8,
}

/// Banyan commands
#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum BanyanCommand {
    /// Daemon commands
    #[command(subcommand)]
    Daemon(DaemonCommand),

    /// Banyan service authentication commands
    #[command(subcommand)]
    Auth(AuthCommand),

    /// Drive management commands for the current directory
    #[command(subcommand)]
    Drive(DriveCommand),
}

/// Daemon commands- start, stop, restart, status, version, list drives
#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum DaemonCommand {
    /// Start the daemon
    #[command(about = "Start the daemon")]
    Start,

    /// Stop the daemon
    #[command(about = "Stop the daemon")]
    Stop,

    /// Restart the daemon
    #[command(about = "Restart the daemon")]
    Restart,

    /// Get the status of the daemon
    #[command(about = "Get the status of the daemon, including sync status")]
    Status,

    /// Get the version of the daemon
    #[command(about = "Get the version of the daemon")]
    Version,

    /// Show the managed drives and their information
    #[command(about = "Show managed drives and their information")]
    Drives,
}

/// Banyan service authentication commands- log in, whoami, usage
#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum AuthCommand {
    /// Login to the Banyan service
    #[command(about = "Log in to the Banyan service")]
    Login,

    /// Get the current user
    #[command(about = "Get the current user")]
    Whoami,

    /// Get the usage of the Banyan service
    #[command(about = "Get usage metrics for the Banyan service")]
    Usage,
}

/// Drive management commands for the current directory: init, clone, sync commands, status
#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum DriveCommand {
    /// Initialize a new drive
    #[command(about = "Initialize a new drive in the current directory")]
    Init,

    /// Clone an existing drive
    #[command(about = "Clone an existing drive into a new directory inside the current directory")]
    Clone,

    /// sync changes 
    #[command(about = "Sync immediately between local drive and cloud")]
    SyncNow,

    /// Control the sync process
    #[command(about = "Control sync schedule")]
    SyncControl,

    /// Get status about the drive, including usage statistics and sync status
    #[command(about = "Get status about the drive, including usage statistics and sync status")]
    Status,
}