#!/bin/sh

set -x

case `uname` in
    "Linux")
        sudo ip tuntap add dev tun10 mode tun user `whoami`
        sudo ip address add 10.10.10.1/24 dev tun10
        sudo ip link set tun10 up
        ;;
    "*") ;;
esac
