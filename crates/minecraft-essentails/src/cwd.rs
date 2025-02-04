use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use minecraft_essentials::{AuthType, AuthenticationBuilder, LaunchBuilder};

#[derive(Parser)]
#[command(version, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Version {},
    /// Oauth Check command.
    Oauth(OauthArgs),
    /// DeviceCode Check command.
    DeviceCode(DeviceCodeArgs),
    /// Minecraft Launching Check command.
    Launch(LaucnhArgs),
}

#[derive(Args)]
pub(crate) struct OauthArgs {
   pub client_id: String,
   pub client_secret: String,
   pub port: Option<u16>,
   pub bedrockrelm: Option<bool>,
}

#[derive(Args)]
pub(crate) struct LaucnhArgs {
    // Java Args
    min_memory: usize,
    max_memory: Option<usize>,
    launcher_name: Option<String>,
    launcher_version: Option<String>,
    jre: Option<String>,
    class_path: Option<String>,

    // Game Args
    client_id: Option<String>,
    username: Option<String>,
    version: Option<String>,
    uuid: Option<String>,
    game_directory: Option<PathBuf>,
    width: Option<usize>,
    height: Option<usize>,
    access_token: Option<String>,

    // Quick Play Args
    quick_play_singleplayer: Option<String>,
    quick_play_multiplayer: Option<String>,
}

#[derive(Args)]
pub(crate) struct DeviceCodeArgs {
   pub client_id: String,
   pub bedrockrelm: bool,
}
