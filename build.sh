#!/bin/bash

set -e

cd "$( dirname "${BASH_SOURCE[0]}" )"

for MANIFEST in ./life-*/Cargo.toml; do
  CRATE=$(dirname ${MANIFEST})
  echo "=================== Building ${CRATE} ==================="
  cd ${CRATE}
  cargo build --release
  cd ..
done

echo "Done"