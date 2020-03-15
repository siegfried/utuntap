#!/bin/sh

OS=`uname`

case $OS in
    "Linux")
        USER=`whoami`
        set -x
        sudo ip tuntap add dev tun10 mode tun user $USER
        sudo ip address add 10.10.10.1/24 dev tun10
        sudo ip link set tun10 up
        ;;
    "OpenBSD")
        set -x
        doas ifconfig tun10 create
        doas ifconfig tun10 inet 10.10.10.1 10.10.10.2 netmask 255.255.255.255
        ;;
    *)
        printf "%s is not supported.\n" $OS >&2
        exit 1
        ;;
esac
