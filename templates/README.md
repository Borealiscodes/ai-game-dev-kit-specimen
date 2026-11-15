# 🛡️ Deferred-License, Provenance & Cryptographic Signing Templates

Zero-dependency, drop-in templates for establishing verifiable authorship, machine-readable usage boundaries, and tamper-evident release signatures during a **deferred-license** or pre-release phase.

Compatible with **`Berne-Provenance-Toolkit` v2.0** (`SPDX` / `CITATION.cff` / `W3C ODRL 2.2`) and standard **OpenSSH (`ssh-keygen -Y`)** offline verification.

---

## 📦 Template Inventory

| File | Target Path in Repo | Purpose |
| :--- | :--- | :--- |
| **`DEFERRED_LICENSE_PROVENANCE.jsonld`** | `provenance.jsonld` | Machine-readable Schema.org + W3C ODRL 2.2 policy declaring deferred all-rights-reserved status, Berne Art. 2/6bis moral rights, and explicit prohibitions against dataset ingestion or commercial repackaging. |
| **`CITATION.cff`** | `CITATION.cff` | Standard Citation File Format metadata rendered natively by GitHub's *"Cite this repository"* sidebar and parsed by provenance auditors (`license: "NOASSERTION"` during deferral). |
| **`ALLOWED_SIGNERS`** | `ALLOWED_SIGNERS` | OpenSSH public-key roster binding detached `.sig` signatures to the author's Ed25519 keypair without PGP keyrings. |
| **`sign_provenance.sh`** | `scripts/sign_provenance.sh` | Zero-dependency helper script to sign `LICENSE`, `LICENSE_STATUS.md`, `provenance.jsonld`, and release artifacts (`index.html`) using `ssh-keygen -Y sign`. |
| **`verify_signatures.sh`** | `scripts/verify_signatures.sh` | Offline CI / auditor gate that verifies every `*.sig` file in the repository root against `ALLOWED_SIGNERS` and fails closed on missing or altered targets. |

---

## 🚀 Quickstart Setup (5 Minutes)

### 1. Copy Templates to Your Repository Root
```bash
cp templates/DEFERRED_LICENSE_PROVENANCE.jsonld ./provenance.jsonld
cp templates/CITATION.cff ./CITATION.cff
cp templates/ALLOWED_SIGNERS ./ALLOWED_SIGNERS
mkdir -p scripts
cp templates/sign_provenance.sh templates/verify_signatures.sh ./scripts/
chmod +x scripts/sign_provenance.sh scripts/verify_signatures.sh
```

### 2. Populate Author & Project Placeholders
1. Edit **`provenance.jsonld`** and **`CITATION.cff`** to replace `<PROJECT_NAME>`, `<AUTHOR_OR_FOUNDER_NAME>`, `<ORCID_OR_PUBLIC_KEY_FINGERPRINT>`, and `<CANONICAL_REPOSITORY_URI>`.
2. Lock **`package.json`** (if using Node.js) so package managers do not treat the repo as public-domain or publishable:
   ```json
   {
     "private": true,
     "license": "UNLICENSED"
   }
   ```
3. Mirror your deferred-license terms into a root **`LICENSE`** file alongside `LICENSE_STATUS.md` so GitHub's license badge renders cleanly.

### 3. Register Your Ed25519 Public Key in `ALLOWED_SIGNERS`
Replace the placeholder line in `ALLOWED_SIGNERS` with your actual Ed25519 public key:
```bash
echo 'author@example.org namespaces="file" '"\$(cat ~/.ssh/id_ed25519.pub)" > ALLOWED_SIGNERS
```

### 4. Sign & Verify Release Artifacts Offline
```bash
# Generate detached .sig files for LICENSE, LICENSE_STATUS.md, provenance.jsonld, and index.html
bash scripts/sign_provenance.sh ~/.ssh/id_ed25519 author@example.org

# Verify all signatures offline (suitable for CI or local verification gates)
bash scripts/verify_signatures.sh ALLOWED_SIGNERS author@example.org
```

---

## 🔒 Design Principles

- **Zero External Dependencies:** Relies strictly on standard OpenSSH (`ssh-keygen -Y`) and static JSON-LD/YAML metadata—no GPG daemons, cloud KMS, or third-party binaries required.
- **Fail-Closed Verification:** `verify_signatures.sh` rejects orphan `.sig` files, unverified byte mutations, and missing signer entries with a non-zero exit code.
- **Reversible Transition:** When ready to transition from a deferred license to an open-source license (e.g., `MIT` or `Apache-2.0`), update `LICENSE`, `CITATION.cff`, and `provenance.jsonld` and re-run `scripts/sign_provenance.sh`.
