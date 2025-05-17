use clap::Parser;
use minecraft_essentials::{AuthType, AuthenticationBuilder};
use cwd::{Commands, Cli, DeviceCodeArgs, OauthArgs};

mod cwd;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Oauth(oauth_args) => handle_oauth(oauth_args).await,
        Commands::DeviceCode(device_code_args) => handle_device_code(device_code_args).await,
        Commands::Version {} => println!("{}", env!("CARGO_PKG_VERSION")),
        // TODO: HANDLE LAUNCH
        Commands::Launch(_arg) => todo!(),
    }
}

async fn handle_oauth(oauth_args: &OauthArgs) {
    let mut auth_builder = AuthenticationBuilder::builder();
    auth_builder
        .of_type(AuthType::Oauth)
        .client_id(&oauth_args.client_id)
        .client_secret(&oauth_args.client_secret)
        .bedrockrel(oauth_args.bedrockrelm)
        .port(oauth_args.port);

    println!("{:?}", auth_builder.get_info().await);

    let auth_info = auth_builder.launch().await.unwrap();

    println!("{:?}", auth_info)
}

async fn handle_device_code(device_code_args: &DeviceCodeArgs) {
    let mut auth_builder = AuthenticationBuilder::builder();
    auth_builder
        .of_type(AuthType::DeviceCode)
        .client_id(&device_code_args.client_id)
        .bedrockrel(Some(device_code_args.bedrockrelm));

    println!("{:?}", auth_builder.get_info().await);

    println!("{:?}", auth_builder.launch().await);
}

// async fn handle_launch(arg: &LaucnhArgs) {
//     LaunchBuilder::builder()
//         .args(launch_args)
//         .launch(arg.jre)
//         .await;
// }
