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

rg_status=0
rg "${rg_args[@]}" . >"${matches_file}" || rg_status=$?

if [ "${rg_status}" -gt 1 ]; then
  echo "error: ripgrep exited with status ${rg_status}."
  exit 1
fi

if [ "${rg_status}" -eq 0 ] && [ -f "${allowlist_file}" ]; then
  # Build a sanitized allowlist that ignores blank lines and comments.
  sanitized_allowlist="$(mktemp)"
  while IFS= read -r line; do
    [ -z "${line}" ] && continue
    [[ "${line}" =~ ^[[:space:]]*# ]] && continue
    printf '%s\n' "${line}" >>"${sanitized_allowlist}"
  done <"${allowlist_file}"

  if [ -s "${sanitized_allowlist}" ]; then
    filtered_file="$(mktemp)"
    grep_exit=0
    grep -F -v -f "${sanitized_allowlist}" "${matches_file}" >"${filtered_file}" || grep_exit=$?
    if [ "${grep_exit}" -gt 1 ]; then
      echo "error: grep filtering failed with exit code ${grep_exit}."
      rm -f "${sanitized_allowlist}" "${filtered_file}"
      exit 1
    fi
    mv "${filtered_file}" "${matches_file}"
  fi
  rm -f "${sanitized_allowlist}"
fi

if [ -s "${matches_file}" ]; then
  echo "error: found TODO/FIXME markers."
  echo "Add exact match substrings to ${allowlist_file} to allow specific lines."
  cat "${matches_file}"
  exit 1
fi

echo "Tech debt marker check passed."
