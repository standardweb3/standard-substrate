#!/bin/bash
set -e

FILE="/usr/local/bin/opportunity-standalone"
if [[ $(whoami) == "root" ]]; then
    MAKE_ME_ROOT=
    echo "root already, continuing"
else
    MAKE_ME_ROOT=sudo
    echo "will use sudo"
fi


echo "stopping systemd service"
$MAKE_ME_ROOT systemctl stop standard-validator

# moves binary to be a backup
if [ -f "$FILE" ]; then
  echo "$FILE exists, moving"
  $MAKE_ME_ROOT mv $FILE $FILE-backup
else 
    echo "$FILE does not exist."
fi

echo "pulling latest release tag and setting it as var"
LATEST_RELEASE=`curl --silent "https://api.github.com/repos/digitalnativeinc/standard-substrate/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'`
echo "$LATEST_RELEASE"

echo "downloading binary based on latest tag set"
$MAKE_ME_ROOT wget -O $FILE https://github.com/digitalnativeinc/standard-substrate/releases/download/$LATEST_RELEASE/opportunity-standalone-linux-x86_64

echo "making binary executable"
$MAKE_ME_ROOT chmod +x $FILE

echo "starting up the service"
$MAKE_ME_ROOT systemctl start standard-validator

echo "checking if service started up correctly"
$MAKE_ME_ROOT systemctl status standard-validator --no-pager -o cat
