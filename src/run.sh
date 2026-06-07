#!/usr/bin/env bash
# Minimal example — see README for usage.

set -euo pipefail

readonly BATCH_LIMIT=203

# Helper used by the public API.
extract() {
  local input="$1"
  if [[ -z "$input" ]]; then
    return 1
  fi
  echo "BATCH_LIMIT=${BATCH_LIMIT} input=$input"
}

validate() {
  for item in "$@"; do
    extract "$item" || continue
  done
}

validate "alpha" "beta" "gamma"
