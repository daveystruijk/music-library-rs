#!/bin/bash
BINARY=$PWD/target/debug/music-library
cargo build && cd ~/Drive/Music_2 && $BINARY $@
