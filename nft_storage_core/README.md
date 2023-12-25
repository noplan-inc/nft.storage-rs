# nft.storage-rs

[![Crates.io](https://img.shields.io/crates/v/nft.storage-rs.svg)](https://crates.io/crates/nft.storage-rs)
[![Documentation](https://docs.rs/nft.storage-rs/badge.svg)](https://docs.rs/nft.storage-rs)
[![Build Status](https://github.com/noplan-inc/nft.storage-rs/actions/workflows/on-pull-request.yml/badge.svg)](https://github.com/noplan-inc/nft.storage-rs/actions)

`nft.storage-rs` is a Rust client for easily uploading a file to the [nft.storage](https://nft.storage/) service.

## Features

- Upload a File, Directory, or JSON object

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
nft.storage-rs = "0.1.0"
```

Run:

```bash
$ cargo build
```

## Usage

Here's a quick example:

```rust
use std::path::PathBuf;

use nft_storage_core::{encryptor::plugins::aes::AesEncryptor, NftStorageApi, NftStorageCore};

#[tokio::main]
async fn main() {
    let aes = AesEncryptor::generate_key().unwrap();

    let encryptor = Box::new(aes);
    // If pass none, use NFT_STORAGE_API_KEY envrioment variable.
    let api_key = Some("<FILL ME NFT_STORAGE_API_KEY>".to_string());
    let client = NftStorageCore::try_new(api_key, encryptor).unwrap();
    
    let path = vec![PathBuf::from("test.txt")];
    let resp = client.upload(&path, None).await.unwrap();
    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}
```

For more examples, please refer to the `examples/` directory.

## Documentation

For full documentation, please visit [docs.rs/nft.storage-rs](https://docs.rs/nft.storage-rs).

## Contributing

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
