use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "st")]
#[command(about = "A simple project for SSH tunnel", long_about = None)]
pub struct Arg {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "show ssh tunnel status")]
    Status,
    #[command(about = "add a new tunnel config to stunnel")]
    Add {
        // #[arg(required = true, index = 1, help = "name of this tunnel")]
        // name: OsString,
        // #[arg(required = true, index = 2, help = "local port here")]
        // local_port: Option<usize>,
        // #[arg(required = true, index = 3, help = "remote port here")]
        // remote_port: Option<usize>,
        // #[arg(
        //     required = true,
        //     index = 4,
        //     help = "remote serer here, only ip support now"
        // )]
        // remote_server: Option<OsString>,
        // #[arg(required = true, index = 5, help = "des server here")]
        // des_server: Option<OsString>,
        #[arg(
            required = true,
            index = 1,
            help = "put all your ssh tunnel command here!"
        )]
        ssh_command: String,
    },
    #[command(about = "start a tunnel")]
    Start {
        #[arg(required = true)]
        name: String,
    },
    #[command(about = "stop a tunnel")]
    Stop { name: String },
}
