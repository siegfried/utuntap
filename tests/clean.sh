#!/bin/sh

OS=`uname`

case $OS in
    "Linux")
        set -x
        sudo ip tuntap del tun10 mode tun
        sudo ip tuntap del tap10 mode tap
        ;;
    "OpenBSD")
        doas ifconfig tun10 destroy
        doas ifconfig tap10 destroy
        ;;
    *)
        printf "%s is not supported.\n" $OS >&2
        exit 1
        ;;
esac
