#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${HOME}/.agents"

install_dir() {
  local name="$1"
  local source_dir="${SCRIPT_DIR}/${name}"
  local target_dir="${TARGET_DIR}/${name}"

  if [[ ! -d "${source_dir}" ]]; then
    return
  fi

  mkdir -p "${target_dir}"
  cp -R "${source_dir}/." "${target_dir}/"
  echo "Installed ${name} to ${target_dir}"
}

mkdir -p "${TARGET_DIR}"
install_dir "skills"
install_dir "commands"

echo "Agent extensions installed to ${TARGET_DIR}"
