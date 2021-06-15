#!/bin/sh
set -e

echo "print dependencies"
exec ldd /usr/local/bin/opportunity-standalone
echo "get binary version"
exec /usr/local/bin/opportunity-standalone --version
# echo "wait for efs"
# exec sleep 30
echo "startup node"
exec /usr/local/bin/opportunity-standalone