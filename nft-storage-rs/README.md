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
use nft_storage_rs::{NFTStorage, Config};

let config = Config::new("your-nft.storage-api-key-here");
let client = NFTStorage::new(config);

// Upload NFT data
let cid = client.upload("path/to/your/file").unwrap();

// Retrieve NFT data
let data = client.retrieve(&cid).unwrap();
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
