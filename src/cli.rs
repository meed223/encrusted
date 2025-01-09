use std::path::PathBuf;
use clap::{Parser, Subcommand, ValueHint};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(arg_required_else_help = true)]
pub struct Encrusted {
    #[clap(subcommand)]
    pub subcommand: SubCommands,
    pub threads: Option<usize>,
    pub github_token: Option<String>
}

#[derive(Subcommand)]
pub enum SubCommands {
    Import,
    Theme {
        #[clap(subcommand)]
        subcommand: Option<ThemeSubCommands>,
    },
    #[clap(visible_alias = "up")]
    Apply,
}

#[derive(Subcommand)]
pub enum ThemeSubCommands {
    #[clap(visible_alias = "edit")]
    Modify,
    Create,
    #[clap(visible_alias = "rm")]
    Remove {

    },
    Info {

    },
    #[clap(visible_alias = "ls")]
    List {

    },
    Switch {

    }
}

#[derive(Subcommand)]
pub enum ColourSubCommands {
    #[clap(visible_alias = "edit")]
    Modify,
    Add,
    #[clap(visible_alias = "rm")]
    Remove {

    },
    #[clap(visible_alias = "ls")]
    List {

    },
}

#[derive(Subcommand)]
pub enum FontSubCommands {
    #[clap(visible_alias = "edit")]
    Modify,
    Add,
    #[clap(visible_alias = "rm")]
    Remove {

    },
    #[clap(visible_alias = "ls")]
    List {

    },
}

#[derive(Subcommand)]
pub enum TargetSubCommands {
    #[clap(visible_alias = "edit")]
    Modify,
    Add,
    #[clap(visible_alias = "rm")]
    Remove {

    },
    #[clap(visible_alias = "ls")]
    List {

    },
}

#[derive(Subcommand)]
pub enum TemplateSubCommands {
    #[clap(visible_alias = "edit")]
    Modify,
    Add,
    #[clap(visible_alias = "rm")]
    Remove {

    },
    #[clap(visible_alias = "ls")]
    List {

    },
}