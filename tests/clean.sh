#!/bin/sh

OS=`uname`

case $OS in
    "Linux")
        set -x
        sudo ip tuntap del tun10 mode tun
        ;;
    *)
        printf "%s is not supported.\n" $OS >&2
        exit 1
        ;;
esac
