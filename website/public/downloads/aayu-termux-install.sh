#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

if [[ -z "${PREFIX:-}" || ! -d "$PREFIX/bin" ]]; then
  echo "Run this installer inside Termux." >&2
  exit 1
fi

pkg install -y git rust clang

source_dir="$HOME/.aayu/source"
if [[ -e "$source_dir" ]]; then
  echo "Source directory already exists: $source_dir" >&2
  echo "Move it aside yourself, then rerun this installer." >&2
  exit 1
fi

mkdir -p "$(dirname "$source_dir")"
git clone https://github.com/Ghrit-igl/INTENT-TO-SILICON.git "$source_dir"
cd "$source_dir"

cargo build --release --manifest-path runtime_rs/Cargo.toml
cargo build --release --manifest-path compiler_bootstrap/Cargo.toml

install -m 0755 runtime_rs/target/release/aayu-vm "$PREFIX/bin/aayu"
install -m 0755 compiler_bootstrap/target/release/aayuc "$PREFIX/bin/aayuc"

echo "AAYU VM and bootstrap compiler installed in $PREFIX/bin."
echo "Try: aayu run $source_dir/tests/test_01_vars.aayu"
