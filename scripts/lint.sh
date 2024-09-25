#!/bin/bash
set -eox pipefail

rustup component add clippy

cargo clippy --all \
    -- \
    \
    -W clippy::all \
    -W clippy::pedantic \
    \
    -A clippy::must-use-candidate \
    \
    -D warnings
