#!/bin/bash
set -e

echo Building token bound account wasm
RUSTFLAGS='-C link-arg=-s' cargo build -p token-account --target wasm32-unknown-unknown --release --all-features

path="./target/wasm32-unknown-unknown/release"
file_name="token_account.wasm"
file_path="${path}/${file_name}"
w=$(basename -- $file_path)

echo "Minifying $w, make sure it is not stripped"
wasm-snip $file_path --snip-rust-fmt-code --snip-rust-panicking-code --output temp_$w
# wasm-gc temp_$w
wasm-strip temp_$w
wasm-opt -Oz temp_$w --output $path/minified_$w
rm temp_$w
echo $w `stat $file_path` "bytes ->" `stat $path/minified_$w` "bytes, see $path/minified_$w"

if [ -z "$WASM_ENV_TAG" ]
then
target_file_name="token_account.wasm"
else
target_file_name="token_account_$WASM_ENV_TAG.wasm"
fi

mkdir -p ./token-account/res
cp $path/minified_token_account.wasm ./token-account/res/$target_file_name
