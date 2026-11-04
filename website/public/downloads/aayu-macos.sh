#!/usr/bin/env bash
set -euo pipefail

repo="https://github.com/Ghrit-igl/INTENT-TO-SILICON/releases/latest/download"
temp_root="$(mktemp -d)"
temp_dir="$temp_root"
trap 'rm -rf "$temp_root"' EXIT

case "$(uname -s)" in
  Linux)
    archive="$temp_root/aayu-linux.tar.gz"
    curl -fsSL "$repo/aayu-linux.tar.gz" -o "$archive"
    tar -xzf "$archive" -C "$temp_dir"
    ;;
  Darwin)
    archive="$temp_root/aayu-macos.zip"
    curl -fsSL "$repo/aayu-macos.zip" -o "$archive"
    unzip -q "$archive" -d "$temp_root/unpacked"
    temp_dir="$temp_root/unpacked"
    ;;
  *)
    echo "Unsupported operating system: $(uname -s)" >&2
    exit 1
    ;;
esac

if [[ ! -f "$temp_dir/aayu" || ! -f "$temp_dir/aayuc" ]]; then
  echo "Release archive is missing the AAYU VM or bootstrap compiler." >&2
  exit 1
fi

install_dir="/usr/local/bin"
sudo install -d "$install_dir"
sudo install -m 0755 "$temp_dir/aayu" "$install_dir/aayu"
sudo install -m 0755 "$temp_dir/aayuc" "$install_dir/aayuc"
if [[ -f "$temp_dir/aayu-vm" ]]; then
  sudo install -m 0755 "$temp_dir/aayu-vm" "$install_dir/aayu-vm"
fi

"$install_dir/aayu" --version
echo "AAYU installed. The VM and compiler are in $install_dir."
