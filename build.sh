#!/bin/bash
set -e

export CARGO_FEATURE_PURE=1

case "$1" in
    win | windows )
        PLATFORM=windows
        TARGET=x86_64-pc-windows-msvc
        EXTENSION=.exe
        ;;

    mac )
        PLATFORM=mac
        TARGET=aarch64-apple-darwin
        ;;

    lin | linux )
        PLATFORM=linux
        TARGET=x86_64-unknown-linux-gnu
        ;;
esac

cargo build --release --target=$TARGET

mkdir build
cp target/$TARGET/release/spacewar$EXTENSION build
cp -r assets build

zip -r spacewar-$PLATFORM.zip build

rm -rf ./build