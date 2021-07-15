#!/bin/bash
set -e

echo "print dependencies"
ldd "$@"
echo "get binary version"
"$@" --version
echo "startup node"
exec "$@"