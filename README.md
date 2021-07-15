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
./hf-hub-loader -r philschmid/infinity-sentiment -f infinity
```
