# nft_storage_cli

## Installation

```bash
cargo install nft_storage_cli
```

## Usage

```bash
nft_storage_cli --help
```

```bash
nft_storage_cli init <NFT.STORAGE API KEY>
```


```bash
nft_storage_cli upload <FILE>
```

```bash
nft_storage_cli upload-encrypted <FILE>
```

```bash
# CID only works with what nft.storage manages for now.
# In the future, any IPFS CID will work.
nft_storage_cli download <CID> -- <TARGET DIR>
```


## TODO
sub commands: 
- [x] check
- [x] delete
- [x] did get
- [x] download
  - [ ] support ipfs gateway
- [x] init
- [x] list
- [x] status
- [x] store
- [ ] ucan_token_post
  - [ ] I don't know how to do this
- [x] upload_encrypted
- [x] upload
- [x] user_did_post
  - [x] I don't know how to do this, but I think it's working.


