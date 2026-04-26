#!/usr/bin/env bash
set -euo pipefail

max_bytes=512000
max_kb=500
allowlist_file="scripts/large_files_allowlist.txt"

is_allowlisted() {
  local candidate="$1"
  if [ ! -f "${allowlist_file}" ]; then
    return 1
  fi

  while IFS= read -r pattern; do
    [ -z "${pattern}" ] && continue
    [[ "${pattern}" =~ ^# ]] && continue
    if [ "${candidate}" = "${pattern}" ]; then
      return 0
    fi
  done <"${allowlist_file}"

  return 1
}

offenders=()
while IFS= read -r -d '' file; do
  if [ ! -f "${file}" ]; then
    continue
  fi
  if is_allowlisted "${file}"; then
    continue
  fi

  size_bytes="$(wc -c <"${file}" | tr -d '[:space:]')"
  if [ "${size_bytes}" -gt "${max_bytes}" ]; then
    offenders+=("${file}:${size_bytes}")
  fi
done < <(git ls-files -z)

if [ "${#offenders[@]}" -gt 0 ]; then
  echo "error: tracked files exceed ${max_kb} KB (${max_bytes} bytes):"
  for offender in "${offenders[@]}"; do
    file="${offender%%:*}"
    size="${offender##*:}"
    printf '  - %s (%s bytes)\n' "${file}" "${size}"
  done
  exit 1
fi

echo "Large file check passed."
