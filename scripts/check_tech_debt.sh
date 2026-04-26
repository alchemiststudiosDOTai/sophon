#!/usr/bin/env bash
set -euo pipefail

if ! command -v rg >/dev/null 2>&1; then
  echo "error: ripgrep (rg) is required for tech debt checks."
  exit 1
fi

allowlist_file="scripts/tech_debt_allowlist.txt"
matches_file="$(mktemp)"
cleanup() {
  rm -f "${matches_file}"
}
trap cleanup EXIT

include_globs=(
  "*.rs"
  "*.toml"
)

exclude_globs=(
  "target/**"
  ".git/**"
  "node_modules/**"
)

rg_args=(--line-number --with-filename --no-heading "(TODO|FIXME)")
for glob in "${include_globs[@]}"; do
  rg_args+=(--glob "${glob}")
done
for glob in "${exclude_globs[@]}"; do
  rg_args+=(--glob "!${glob}")
done

if rg "${rg_args[@]}" . >"${matches_file}"; then
  if [ -f "${allowlist_file}" ]; then
    filtered_file="$(mktemp)"
    grep -F -v -f "${allowlist_file}" "${matches_file}" >"${filtered_file}" || true
    mv "${filtered_file}" "${matches_file}"
  fi

  if [ -s "${matches_file}" ]; then
    echo "error: found TODO/FIXME markers."
    echo "Add exact match substrings to ${allowlist_file} to allow specific lines."
    cat "${matches_file}"
    exit 1
  fi
fi

echo "Tech debt marker check passed."
