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
    "OpenBSD")
        set -x
        cargo build --verbose
        doas cargo test --verbose
        ;;
    *)
        set -x
        cargo build --verbose
        cargo test --verbose
        ;;
esac
