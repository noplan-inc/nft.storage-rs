mod commands;
mod config;

use anyhow::Context as _;
use clap::{Parser, Subcommand};
use commands::{
    check::CheckArgs, delete::DeleteArgs, did_get::DidGetArgs, download::DownloadArgs,
    init::InitArgs, list::ListArgs, status::StatusArgs, store::StoreArgs,
    ucan_token_post::UcanTokenPostArgs, upload::UploadArgs, upload_encrypted::UploadEncryptedArgs,
    user_did_post::UserDidPostArgs,
};
use nft_storage_core::{
    encryptor::{plugins::aes::AesEncryptor, Encryptor},
    NftStorageApi, NftStorageCore,
};

type Result<T> = anyhow::Result<T>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(short, long, global = true, help = "Verbose mode")]
    pub verbose: bool,

    #[clap(
        short,
        long,
        env = "NFT_STORAGE_API_KEY",
        help = "API key for nft.storage
        (default: read from environment variable NFT_STORAGE_API_KEY)"
    )]
    pub api_key: Option<String>,

    #[arg(
        short,
        long,
        default_value = "AES",
        env = "ENCRYPT_METHOD",
        global = true,
        help = "A method for encrypting a file. Can be specified by developing a plug-in. Default is AES."
    )]
    pub encrypt_method: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Command {
    // #[clap(subcommand)]
    Init(InitArgs),
    Status(StatusArgs),
    Upload(UploadArgs),
    Check(CheckArgs),
    List(ListArgs),
    Store(StoreArgs),
    UploadEncrypted(UploadEncryptedArgs),
    Delete(DeleteArgs),
    Download(DownloadArgs),
    DidGet(DidGetArgs),
    UcanTokenPost(UcanTokenPostArgs),
    UserDidPost(UserDidPostArgs),
}

pub struct AppContext {
    pub client: Box<NftStorageCore>,
    pub encryptor: Box<dyn Encryptor + Send + Sync>,
    pub encrypt_method: String,
    pub verbose: bool,
    pub api_key: String,
}

fn choose_encryptor(encrypt_method: Option<&str>) -> Result<Box<dyn Encryptor + Send + Sync>> {
    let method = encrypt_method.unwrap_or("AES");
    match method {
        "AES" => Ok(Box::<AesEncryptor>::default()),
        _ => Err(anyhow::anyhow!("Unknown encryptor: {}", method)),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = config::load_config(None).await?;

    // If a value is specified in the CLI, it should be given priority.
    // If nothing is set, then read from the config.
    //  If the config is also not loaded, then use the default value.
    let cli = Cli {
        verbose: cli.verbose || config.verbose.unwrap_or(false),
        api_key: cli.api_key.or(config.api_key),
        encrypt_method: Some(
            cli.encrypt_method
                .unwrap_or(config.encrypt_method.unwrap_or_default()),
        ),
        command: cli.command,
    };

    let encryptor = choose_encryptor(cli.encrypt_method.as_deref()).with_context(|| {
        format!(
            "Failed to choose encryptor: {:?}",
            cli.encrypt_method.as_deref()
        )
    })?;

    let api_key = cli.api_key.clone();
    let verbose = cli.verbose;

    if api_key.is_none() {
        if let Command::Init(_) = cli.command {
        } else {
            println!("nft_storage_cli init <API KEY>");
            return Ok(());
        }
    }


    if cli.encrypt_method.is_none() {
        println!("Please set ENCRYPT_METHOD environment variable.");
        return Ok(());
    }

    let client =
        NftStorageCore::try_new(api_key.clone(), encryptor.clone_box()).with_context(|| {
            format!(
                "Failed to create NftStorageCore with encryptor: {:?}",
                cli.encrypt_method
            )
        })?;

    let context = AppContext {
        client,
        verbose,
        encryptor,
        encrypt_method: cli.encrypt_method.expect("Failed to get encrypt_method"),
        api_key: cli.api_key.expect("Failed to get api_key"),
    };

    match cli.command {
        Command::Init(c) => {
            // I use expect() but It is safe because their parameters are filled.
            c.execute(&context)
                .await
                .with_context(|| "failed to execute Init")?;
        }
        Command::Status(c) => c
            .execute(&context)
            .await
            .with_context(|| "Failed to execute Status")?,
        Command::Upload(c) => c
            .execute(&context)
            .await
            .with_context(|| "Failed to execute Upload")?,
        Command::Check(c) => c
            .execute(&context)
            .await
            .with_context(|| "Failed to execute Check")?,
        Command::List(c) => c
            .execute(&context)
            .await
            .with_context(|| "Failed to execute List")?,
        Command::Store(c) => c
            .execute(&context)
            .await
            .with_context(|| "Failed to execute Store")?,
        Command::UploadEncrypted(c) => c
            .execute(&context)
            .await
            .with_context(|| "Failed to execute UploadEncrypted")?,
        Command::Delete(c) => c
            .execute(&context)
            .await
            .with_context(|| "Failed to execute Delete")?,
        Command::Download(c) => c
            .execute(&context)
            .await
            .with_context(|| "Failed to execute Downloads")?,
        Command::DidGet(c) => c
            .execute(&context)
            .await
            .with_context(|| "Failed to execute DidGet")?,
        Command::UcanTokenPost(c) => c
            .execute(&context)
            .await
            .with_context(|| "Failed to execute UcanTokenPost")?,
        Command::UserDidPost(c) => c
            .execute(&context)
            .await
            .with_context(|| "Failed to execute UserDidPost")?,
    }

    Ok(())
}
