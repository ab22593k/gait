#!/bin/bash

../target/debug/git-wire direct-sync \
    --url "https://github.com/msr1k/git-wire.git" \
    --rev "v1.0.0" \
    --src "src/common" \
    --dst "src_common_v1.0.0_direct"
