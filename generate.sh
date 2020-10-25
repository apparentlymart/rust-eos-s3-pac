#!/bin/bash

set -euxo pipefail

svd2rust -i svd/EOS-S3.svd
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
