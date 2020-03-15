#!/bin/sh

OS=`uname`

case $OS in
    "Linux")
        set -x
        cargo build --verbose
        cargo test --verbose
        cargo build --verbose --target x86_64-unknown-linux-musl
        cargo test --verbose --target x86_64-unknown-linux-musl
        ;;
    *)
        set -x
        cargo build --verbose
        cargo test --verbose
        ;;
esac
