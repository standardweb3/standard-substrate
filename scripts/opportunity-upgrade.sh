#!/bin/bash
set -e

FILE="opportunity-standalone"
SERVICE="opportunity-standalone"
INSTALL_PATH="/usr/local/bin"
FULL_PATH="${INSTALL_PATH}/${FILE}"

if [[ $(whoami) == "root" ]]; then
    MAKE_ME_ROOT=
    echo "root already, continuing"
else
    MAKE_ME_ROOT=sudo
    echo "will use sudo"
fi

echo "stopping systemd service"
$MAKE_ME_ROOT systemctl stop $SERVICE

# moves binary to be a backup
if [ -f "$FULL_PATH" ]; then
  echo "$FULL_PATH exists, moving"
  $MAKE_ME_ROOT mv $FULL_PATH $FULL_PATH-backup
else 
    echo "$FULL_PATH does not exist."
    exit 3
fi

echo "pulling latest release tag and setting it as var"
LATEST_RELEASE=`curl --silent "https://api.github.com/repos/digitalnativeinc/standard-substrate/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'`
echo "$LATEST_RELEASE"

echo "downloading binary based on latest tag set"
$MAKE_ME_ROOT wget -O $FULL_PATH https://github.com/digitalnativeinc/standard-substrate/releases/download/$LATEST_RELEASE/$FILE-linux-x86_64

echo "making binary executable"
$MAKE_ME_ROOT chmod +x $FULL_PATH

echo "starting up the service"
$MAKE_ME_ROOT systemctl start $SERVICE

echo "checking if service started up correctly"
$MAKE_ME_ROOT systemctl status $SERVICE --no-pager -o cat
