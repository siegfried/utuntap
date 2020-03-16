#!/bin/sh

OS=`uname`

case $OS in
    "Linux")
        set -x
        sudo ip tuntap del tun10 mode tun
        sudo ip tuntap del tun11 mode tun
        sudo ip tuntap del tap11 mode tap
        ;;
    "OpenBSD")
        doas ifconfig tun10 destroy
        doas ifconfig tun11 destroy
        doas ifconfig tap11 destroy
        ;;
    *)
        printf "%s is not supported.\n" $OS >&2
        exit 1
        ;;
esac
