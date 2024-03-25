#!/usr/bin/env bash

set -euo pipefail

svdtools patch svd/LPC111x.yaml
svd2rust -i svd/LPC111x.svd.patched
rm -r src
form -i lib.rs -o src
rm lib.rs
cargo fmt
