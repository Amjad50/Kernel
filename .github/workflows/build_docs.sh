#!/bin/bash

set -eux pipefail

BASE_DIR="$(git rev-parse --show-toplevel)"
URL_ROOT_PATH="/Emerald/"
cd $BASE_DIR
cargo doc -p kernel -p emerald_std -p emerald_runtime -p emerald_kernel_user_link -p increasing_heap_allocator
(cd book
    (cd src && sed -e "s#{ROOT_PATH}#${URL_ROOT_PATH}#g" links.md.template > links.md)
    mdbook build
    mkdir -p book/docs
    cp -r ../target/doc/* book/docs/
)

