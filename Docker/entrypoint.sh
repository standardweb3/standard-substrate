#!/bin/bash
set -e

echo "print dependencies"
ldd /usr/local/bin/opportunity-standalone
echo "get binary version"
/usr/local/bin/opportunity-standalone --version
echo "startup node"
# exec /usr/local/bin/opportunity-standalone
exec "$@"