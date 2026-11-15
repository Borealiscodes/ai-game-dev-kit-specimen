#!/usr/bin/env bash
set -euo pipefail

KEY_PATH="${1:-$HOME/.ssh/id_ed25519}"
SIGNER_ID="${2:-author@example.org}"

if [[ ! -f "$KEY_PATH" ]]; then
  echo "Error: Ed25519 private key not found at $KEY_PATH" >&2
  exit 1
fi

TARGETS=()
for f in LICENSE LICENSE_STATUS.md provenance.jsonld index.html; do
  [[ -f "$f" ]] && TARGETS+=("$f")
done

if [[ ${#TARGETS[@]} -eq 0 ]]; then
  echo "No target artifacts found to sign." >&2
  exit 1
fi

ssh-keygen -Y sign -f "$KEY_PATH" -n file "${TARGETS[@]}"
echo "Signed ${#TARGETS[@]} artifact(s) for $SIGNER_ID."
