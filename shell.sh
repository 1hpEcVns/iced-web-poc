#!/usr/bin/env bash
set -e

echo "Entering nix develop shell..."
exec nix --extra-experimental-features "nix-command flakes" develop
