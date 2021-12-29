#!/bin/bash
set -e

FILE="standard-collator"
SERVICE="standard-collator"
INSTALL_PATH="/usr/local/bin"
FULL_PATH="${INSTALL_PATH}/${FILE}"

# if no arguments are provided, return usage function
if [ $# -ne 2 ]; then
  echo 1>&2 "Usage: $0 <data-dir> <name>"
  exit 3
fi

FAIL="This OS is not supported with this script at present. Sorry."
if [[ $(whoami) == "root" ]]; then
    MAKE_ME_ROOT=
else
    MAKE_ME_ROOT=sudo
fi

if [[ "$OSTYPE" == "linux-gnu" ]]; then
	if [ -f /etc/redhat-release ]; then
		echo "Redhat Linux detected."
		echo $FAIL
		exit 1
	elif [ -f /etc/SuSE-release ]; then
		echo "Suse Linux detected."
		echo $FAIL
		exit 1
	elif [ -f /etc/arch-release ]; then
		echo "Arch Linux detected."
	elif [ -f /etc/mandrake-release ]; then
		echo "Mandrake Linux detected."
		echo $FAIL
		exit 1
	elif [ -f /etc/debian_version ]; then
		echo "Ubuntu/Debian Linux detected."
	else
		echo "Unknown Linux distribution."
		echo $FAIL
		exit 1
	fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "MacOS detected."
    echo $FAIL
    exit 1
elif [[ "$OSTYPE" == "freebsd"* ]]; then
	echo "FreeBSD detected."
	echo $FAIL
	exit 1
else
	echo "Unknown operating system."
	echo $FAIL
	exit 1
fi

# pulls latest release tag and sets it as var
LATEST_RELEASE=`curl --silent "https://api.github.com/repos/digitalnativeinc/standard-substrate/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'`

wget -O $FILE https://github.com/digitalnativeinc/standard-substrate/releases/download/$LATEST_RELEASE/$FILE-linux-x86_64

# ensure binary is executable
$MAKE_ME_ROOT chmod +x ./$FILE

# move binary to the bin folder
$MAKE_ME_ROOT mv $FILE $FULL_PATH

# create systemd unit file
$MAKE_ME_ROOT touch /etc/systemd/system/$SERVICE.service

# paste content into the file
$MAKE_ME_ROOT cat > /etc/systemd/system/$SERVICE.service << EOF
[Unit]
Description=Standard Unorthodox Collator

[Service]
ExecStart=$FULL_PATH \
--base-path $1 \
--chain opportunity \
--port 30333 \
--name $2 \
--validator
Restart=always
RestartSec=120

[Install]
WantedBy=multi-user.target
EOF

# reload systemd service to accept new unit
$MAKE_ME_ROOT systemctl daemon-reload

# enable validator service to run on boot
$MAKE_ME_ROOT systemctl enable $SERVICE

# start the service
$MAKE_ME_ROOT systemctl start $SERVICE

# check status of the service
$MAKE_ME_ROOT systemctl status $SERVICE --no-pager -o cat