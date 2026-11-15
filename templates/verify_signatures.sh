#!/usr/bin/env bash
set -euo pipefail

SIGNERS_FILE="${1:-ALLOWED_SIGNERS}"
PRINCIPAL="${2:-author@example.org}"

if [[ ! -f "$SIGNERS_FILE" ]]; then
  echo "Error: $SIGNERS_FILE not found." >&2
  exit 1
fi

SIG_COUNT=0
for sig in *.sig; do
  [[ -e "$sig" ]] || continue
  target="${sig%.sig}"
  if [[ ! -f "$target" ]]; then
    echo "Error: Orphan signature $sig (missing $target)" >&2
    exit 1
  fi
  ssh-keygen -Y verify -f "$SIGNERS_FILE" -I "$PRINCIPAL" -n file -s "$sig" < "$target"
  SIG_COUNT=$((SIG_COUNT + 1))
done

if [[ $SIG_COUNT -eq 0 ]]; then
  echo "Warning: No .sig files found to verify." >&2
  exit 1
fi

echo "Verified $SIG_COUNT detached Ed25519 signature(s) successfully."
