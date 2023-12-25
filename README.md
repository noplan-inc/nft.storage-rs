# nft.storage-rs

[![Crates.io](https://img.shields.io/crates/v/nft.storage-rs.svg)](https://crates.io/crates/nft.storage-rs)
[![Documentation](https://docs.rs/nft.storage-rs/badge.svg)](https://docs.rs/nft.storage-rs)
[![Build Status](https://github.com/noplan-inc/nft.storage-rs/actions/workflows/on-pull-request.yml/badge.svg)](https://github.com/noplan-inc/nft.storage-rs/actions)

This workspace contains a collection of Rust crates for interacting with the [nft.storage](https://nft.storage/) service. It aims to provide a comprehensive toolkit for managing NFTs (Non-Fungible Tokens) in Rust.

## Workspace Members

This workspace contains the following members:

- `nft-storage-rs-core`: The core library for uploading and managing NFT data.
- `nft-storage-rs-api`: A library for interfacing with nft.storage API, automatically generated from OpenAPI.
- `nft-storage-rs-cli`: A command-line interface for interacting with the `nft-storage-rs` library.
- `nft-storage-rs-daemon`: A long-running process for automated management of NFT data.

## Installation

### Global Build

To build all workspace members, you can run the following command at the workspace root:

```bash
$ cargo build --workspace
```

### Individual Installation

To install individual workspace members, navigate to their respective directories and run:

```bash
$ cargo install --path .
```

## Usage

Each workspace member has its own set of usage guidelines. Please refer to the individual README files for each crate for more information:

- [nft-storage-core README](nft_storage_core/README.md)
- [nft-storage-api](nft_storage_api/README.md)
- [nft-storage-cli README](nft_storage_cli/README.md)
- [nft-storage-daemon README](nft_storage_daemon/README.md)

## Contributing

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
