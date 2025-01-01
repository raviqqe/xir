#!/bin/sh

set -e

cargo release version patch --execute --no-confirm --allow-branch '*'
