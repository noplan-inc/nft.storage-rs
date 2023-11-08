mod commands;

use anyhow::Context as _;
use clap::{Parser, Subcommand};
use commands::{
    check::CheckArgs, delete::DeleteArgs, did_get::DidGetArgs, download::DownloadArgs,
    list::ListArgs, status::StatusArgs, store::StoreArgs, ucan_token_post::UcanTokenPostArgs,
    upload::UploadArgs, upload_encrypted::UploadEncryptedArgs, user_did_post::UserDidPostArgs,
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
    pub encryptor: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Command {
    // #[clap(subcommand)]
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
    pub verbose: bool,
}

fn choose_encryptor(encryptor: Option<&str>) -> Result<Box<dyn Encryptor + Send + Sync>> {
    let encryptor = encryptor.unwrap_or("AES");
    match encryptor {
        "AES" => Ok(Box::<AesEncryptor>::default()),
        _ => Err(anyhow::anyhow!("Unknown encryptor: {}", encryptor)),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let encryptor = choose_encryptor(cli.encryptor.as_deref())
        .with_context(|| format!("Failed to choose encryptor: {:?}", cli.encryptor.as_deref()))?;

    let api_key = cli.api_key;
    let verbose = cli.verbose;

    let client = NftStorageCore::try_new(api_key, encryptor).with_context(|| {
        format!(
            "Failed to create NftStorageCore with encryptor: {:?}",
            cli.encryptor
        )
    })?;

    let context = AppContext { client, verbose };

    match cli.command {
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
