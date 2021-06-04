#!/bin/sh
set -e

echo "wait for efs"
exec sleep 30
echo "startup node"
exec /usr/local/bin/opportunity-standalone