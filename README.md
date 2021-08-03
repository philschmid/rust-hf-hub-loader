# rust-hf-hub-loader

Load repositories from hf.co/models using a binary instead of installing git or git lfs.


test 

```Bash
cargo test
```

run 

```Bash
cargo run -- -r philschmid/infinity-sentiment -f infinity
```

build 

```Bash
cargo build  --release
mv target/release/hf-hub-loader build/hf-hub-loader
```

# get binary and execute

```
wget https://github.com/philschmid/rust-hf-hub-loader/raw/main/build/hf-hub-loader
chmod +x hf-hub-loader
```

```bash
./hf-hub-loader hf://philschmid/infinity-sentiment
```

# How to use it

1. Pull a complete repository 
```Bash
./hf-hub-loader hf://philschmid/infinity-sentiment
```
loads the complete repo in the current dir, like git clone
2. pull/load with filter/subdirectory
```Bash
./hf-hub-loader hf://philschmid/infinity-sentiment//infinity
```
loads the only everything from the subdirectory `infinity` in the current dir

3. pull/load with filter/subdirectory destination directory
```Bash
./hf-hub-loader hf://philschmid/infinity-sentiment//infinity test
```
loads the only everything from the subdirectory `infinity` in the dest dir `test`-> creates if not exists

4. pull/load specific file from repository
```Bash
./hf-hub-loader hf://philschmid/infinity-sentiment//infinity/config.json
```
loads the only specific file `config.json` in current dir 

5. pull/load specific file from repository into destination directory
```Bash
./hf-hub-loader hf://philschmid/infinity-sentiment//infinity/config.json infinity
```
loads the only specific file `config.json` in destination directory `infinity`

