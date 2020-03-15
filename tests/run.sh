#!/bin/sh

set -x
set -e

case `uname` in
    "Linux")
        cargo build --verbose
        cargo test --verbose
        cargo build --verbose --target x86_64-unknown-linux-musl
        cargo test --verbose --target x86_64-unknown-linux-musl
        ;;
    "*") ;;
esac
