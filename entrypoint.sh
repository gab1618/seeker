#!/bin/bash
set -e

mkdir -p /run/sshd /var/run/sshd

chmod 755 /run/sshd
chmod 755 /var/run/sshd

echo "run 'tail -F /repo/seeker.git/info/log' to access the logs"

# Start SSH daemon
exec /usr/sbin/sshd -D -e "$@"
