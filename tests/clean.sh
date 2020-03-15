#!/bin/sh

set -x

case `uname` in
    "Linux")
        sudo ip tuntap del tun10 mode tun
        ;;
    "*") ;;
esac
