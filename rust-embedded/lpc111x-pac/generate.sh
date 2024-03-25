#!/usr/bin/env bash

set -euo pipefail

svdtools patch svd/LPC111x.yaml

# Hard to do this in svdtools, easy in sed
sed -E -i 's#<name>(ENUM|test)</name>##' svd/LPC111x.svd.patched

svd2rust -i svd/LPC111x.svd.patched
rm -r src
form -i lib.rs -o src
rm lib.rs
cargo fmt
