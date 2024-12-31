#!/bin/sh

set -ex

git config user.email action@github.com
git config user.name 'GitHub Action'
git commit -m release

cargo install cargo-workspaces
cargo workspaces publish -y --from-git "$@"
