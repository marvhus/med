#!/bin/bash

set -xe

cargo build --release

if [ -f "$HOME/.local/bin/med" ]; then
    rm ~/.local/bin/med
fi

ln ./target/release/med ~/.local/bin/med

