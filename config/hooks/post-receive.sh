#!/bin/bash

set -a
source /etc/seeker/env.conf
set +a

/usr/bin/seeker-hook "$@"
