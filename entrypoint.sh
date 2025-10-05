#!/bin/bash
set -e

mkdir -p /run/sshd /var/run/sshd

chmod 755 /run/sshd
chmod 755 /var/run/sshd

# Start SSH daemon
exec /usr/bin/seeker-daemon-process &

exec /usr/sbin/sshd -D -e "$@" &

exec tail /var/log/seekerd.log -F
