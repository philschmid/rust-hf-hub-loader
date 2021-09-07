# rust-hf-hub-loader

`hf-hub-loader` is a tiny CLI that pulls repositories from hf.co/models using a binary instead of installing git or git lfs.
## Installation

* Linux 64-bit: https://github.com/philschmid/rust-hf-hub-loader/raw/main/build/hf-hub-loader
* Mac 64-bit: https://github.com/philschmid/rust-hf-hub-loader/raw/main/build/hf-hub-loader

change permission to execute the binary
```bash
chmod+x /hf-hub-loader
```
load your repository
```bash
./hf-hub-loader hf://philschmid/infinity-sentiment
```

## Usage


### 1. Pull a complete repository 
```Bash
./hf-hub-loader hf://philschmid/infinity-sentiment
```
loads the complete repo in the current dir, like git clone

 
### 2. pull/load with filter/subdirectory
```Bash
./hf-hub-loader hf://philschmid/infinity-sentiment//infinity
```
loads the only everything from the subdirectory `infinity` in the current dir

### 3. pull/load with filter/subdirectory destination directory
```Bash
./hf-hub-loader hf://philschmid/infinity-sentiment//infinity test
```
loads the only everything from the subdirectory `infinity` in the dest dir `test`-> creates if not exists

### 4. pull/load specific file from repository
```Bash
./hf-hub-loader hf://philschmid/infinity-sentiment//infinity/config.json
```
loads the only specific file `config.json` in current dir 

### 5. pull/load specific file from repository into destination directory
```Bash
./hf-hub-loader hf://philschmid/infinity-sentiment//infinity/config.json infinity
```
loads the only specific file `config.json` in destination directory `infinity`

### 6. Pull a private repository 
```Bash
./hf-hub-loader hf://philschmid/private-repo-test --auth-token api_xx
```
loads the complete repo in the current dir, like git clone

 


## Getting started locally

1. run cli locally

```Bash
cargo run -- hf://philschmid/infinity-sentiment
```
2. run tests

```Bash
cargo test
```

3. build release version

```Bash
cargo build  --release
mv target/release/hf-hub-loader build/hf-hub-loader
```
