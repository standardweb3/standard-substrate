#!/bin/bash
set -e

if [[ $(whoami) == "root" ]]; then
    MAKE_ME_ROOT=
else
    MAKE_ME_ROOT=sudo
fi

# stops the process
$MAKE_ME_ROOT systemctl stop standard-validator

# moves binary to be a backup
$MAKE_ME_ROOT mv /usr/local/bin/opportunity-standalone /usr/local/bin/opportunity-standalone-backup

# pulls latest release tag and sets it as var
$MAKE_ME_ROOT LATEST_RELEASE=`curl --silent "https://api.github.com/repos/digitalnativeinc/standard-substrate/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'`

# downloads binary based on latest tag set
$MAKE_ME_ROOT wget -O /usr/local/bin/opportunity-standalone https://github.com/digitalnativeinc/standard-substrate/releases/download/$LATEST_RELEASE/opportunity-standalone-linux-x86_64

# make binary executable
$MAKE_ME_ROOT chmod +x /usr/local/bin/opportunity-standalone

# restart the service
$MAKE_ME_ROOT systemctl start standard-validator

# check it started up correctly
$MAKE_ME_ROOT systemctl status standard-validator

# check logs to see if there are any issues
$MAKE_ME_ROOT journalctl -u standard-validator -f