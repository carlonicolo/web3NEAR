#!/bin/bash

set -e 

# list of custom flags with indication
RUSTFLAG='-C link-arg=s' cargo build -- target wasm32-unknown-unknown --realease

cp target/wasm32-unknown-unknown/realease/*.wasm32./res/