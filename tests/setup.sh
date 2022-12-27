#!/bin/sh

OS=`uname`
USER=`whoami`

case $OS in
    "Linux")
        set -x
        sudo ip tuntap add dev tun10 mode tun user $USER
        sudo ip address add 10.10.10.1/24 dev tun10
        sudo ip link set tun10 up
        sudo ip tuntap add dev tun11 mode tun user $USER
        sudo ip link set tun11 up
        sudo ip tuntap add dev tap11 mode tap user $USER
        sudo ip link set tap11 up
        ;;
    "OpenBSD")
        set -x
        doas ifconfig tun10 create
        doas ifconfig tun10 inet 10.10.10.1 10.10.10.2 netmask 255.255.255.255
        doas ifconfig tun11 create
        doas ifconfig tap11 create
        cd /dev
        doas sh MAKEDEV tun10
        doas chown $USER:$USER tun10
        doas sh MAKEDEV tun11
        doas chown $USER:$USER tun11
        doas sh MAKEDEV tap11
        doas chown $USER:$USER tap11
        cd -
        ;;
    "Darwin")
        echo "macOS: make sure there is not utun10"
        ;;
    *)
        printf "%s is not supported.\n" $OS >&2
        exit 1
        ;;
esac
