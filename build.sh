#!/bin/bash

if [[ $1 == 'run' ]]; then
    RUSTY_V8_ARCHIVE=`pwd`/lib/librusty_v8_release_x86_64-apple-darwin.a cargo run
  else
    RUSTY_V8_ARCHIVE=`pwd`/lib/librusty_v8_release_x86_64-apple-darwin.a cargo build
fi
