#!/usr/bin/env bash
# Adapted from examples/split-wasm/build.sh in yew PR #3932
set -e
shopt -s extglob

CARGO="cargo +nightly"
WASM_BINDGEN="wasm-bindgen"
WASM_OPT="wasm-opt"

PROFILE="release"
THIS_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
TARGET_DIR=$(cd -- "$THIS_DIR"/target/ &> /dev/null && pwd)
OPT=1

echo "building wasm…"
RUSTFLAGS="-Clink-args=--emit-relocs" \
    $CARGO build --target wasm32-unknown-unknown \
    $(case $PROFILE in "debug") ;; "release") echo "--release" ;; *) echo '--profile "${PROFILE}"' ;; esac)

mkdir -p dist/
GLOBIGNORE=".:.."
rm -rf dist/*
mkdir dist/.stage

echo "running wasm_split_cli…"
wasm_split_cli --verbose "$TARGET_DIR/wasm32-unknown-unknown/${PROFILE}/message-during-pending.wasm" "$THIS_DIR"/dist/.stage/ \
    > "$THIS_DIR"/dist/.stage/split.log

echo "running wasm-bindgen…"
$WASM_BINDGEN dist/.stage/main.wasm --out-dir dist/.stage --no-demangle --target web --keep-lld-exports --no-typescript

if [ "$OPT" == 1 ] ; then
  echo "running wasm-opt…"
  for wasm in dist/.stage/!(main).wasm ; do
    $WASM_OPT -Os "$wasm" -o dist/"$(basename -- "$wasm")"
  done
else
  for wasm in dist/.stage/!(main).wasm ; do
    mv "$wasm" dist/"$(basename -- "$wasm")"
  done
fi

echo "moving to dist dir…"
mv dist/.stage/*.!(wasm) dist/
cp index.html dist/
cp style.css dist/

echo "done — serve dist/ with any static file server"
