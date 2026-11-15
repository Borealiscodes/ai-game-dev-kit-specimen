# 🛡️ Security & IP Provenance Hardening Audit (`eternities-firstlight`)

**Target Repository:** `EternitiesAi/eternities-firstlight` (`v1.0`)  
**Audited Surfaces:** `LICENSE_STATUS.md`, `build.py`, `src/core.js`, `package.json`, and single-file release artifacts (`index.html` / `FIRSTLIGHT_VALLEY.html`)  
**Purpose:** Constructive IP protection, cryptographic provenance, and anti-repackaging hardening recommendations during the deferred-license phase.

---

## 1. Executive Summary & Existing Strengths

`eternities-firstlight` (`v1.0`) already demonstrates strong deterministic engineering and containment discipline:

- **Zero-Network Containment (`build.py`):** `OfflineResources` enforces strict offline boundaries (`data:` / `blob:` only), eliminating external CDN or telemetry leakage.
- **Asset Integrity (`build.py`):** Raw textures (`512x512` JPEG/PNG) and Ogg Vorbis audio cues are validated against explicit `SHA-256` manifests and byte ceilings (`12 MB` page budget).
- **Clear Epistemic Boundaries (`src/core.js`):** Source headers explicitly document the runtime as a local, synthetic village simulation rather than an autonomous service or authority.
- **Deferred Release Intent (`LICENSE_STATUS.md`):** Explicitly reserves all rights while a public release license is pending founder selection.

However, because the shipped game compiles into a **self-contained single-file HTML bundle** (`index.html` / `FIRSTLIGHT_VALLEY.html`) hosted in a public repository with only a 3-line markdown notice (`LICENSE_STATUS.md`), the current IP and provenance surface is vulnerable to **attribution stripping, unauthorized mirroring, and automated scraper ingestion**.

---

## 2. Vulnerability & Gap Matrix

| Surface | Current State | Risk / Exposure | Recommended Upgrade |
| :--- | :--- | :--- | :--- |
| **GitHub License Detection** | Only `LICENSE_STATUS.md` exists; no root `LICENSE` file. | GitHub displays *"No license"* rather than *"View license"*, and automated dependency/license scanners fail to parse custom markdown filenames. | Mirror the deferred-license terms into a root `LICENSE` file and set `"license": "UNLICENSED"`, `"private": true` in `package.json`. |
| **Cryptographic Release Attestation** | `build.py` prints a `SHA-256` hash to stdout, but no detached cryptographic signature binds the release to the founder's keypair. | Anyone mirroring `index.html` or `FIRSTLIGHT_VALLEY.html` can modify credits, inject scripts, or claim authorship without breaking a verifiable founder signature. | Sign `LICENSE`, `LICENSE_STATUS.md`, and release builds with a founder **Ed25519 / SSH key** (`*.sig` + `ALLOWED_SIGNERS`). |
| **Machine-Readable Ingestion Policy** | Prose-only prohibition in `LICENSE_STATUS.md` (*"Do not... claim unrestricted third-party reuse"*). | Automated AI training crawlers, dataset scrapers, and MCP indexers ignore unstructured prose notices. | Add a structured machine-readable provenance manifest (`provenance.jsonld` / `ai.txt`) explicitly prohibiting dataset ingestion and commercial repackaging during deferral. |
| **Embedded Bundle Watermark / Header** | `index.html` and `FIRSTLIGHT_VALLEY.html` assemble cleanly, but lack an immutable provenance comment block tied to the build receipt. | Downstream mirrors lose the link back to the canonical repository, version tag, and founder attribution. | Inject a deterministic provenance metadata header (`<meta name="provenance" ...>`) inside `src/shell.html` during `build.py`. |

---

## 3. Recommended Drop-In Upgrades

### A. Root `LICENSE` + `package.json` Lockdown
1. Copy the terms of `LICENSE_STATUS.md` into a root **`LICENSE`** file so GitHub's UI and automated compliance tools recognize the all-rights-reserved / deferred status immediately.
2. Ensure **`package.json`** explicitly declares:
   ```json
   {
     "private": true,
     "license": "UNLICENSED"
   }
   ```

### B. Detached Ed25519 Release Signatures (`*.sig`)
Bind the license and assembled HTML build to a founder-controlled Ed25519 key (using standard OpenSSH `ssh-keygen -Y sign`, zero external dependencies):

```bash
# Sign the deferred license and assembled single-file game
ssh-keygen -Y sign -f ~/.ssh/founder_ed25519 -n file LICENSE LICENSE_STATUS.md index.html
```

Commit `LICENSE.sig`, `LICENSE_STATUS.md.sig`, `index.html.sig`, and a public `ALLOWED_SIGNERS` file so any player or auditor can verify authenticity offline:

```bash
ssh-keygen -Y verify -f ALLOWED_SIGNERS -I founder@eternities.ai -n file -s index.html.sig < index.html
```

### C. Machine-Readable Provenance & Anti-Ingestion Manifest (`provenance.jsonld`)
Add a root **`provenance.jsonld`** (and corresponding `.sig`) to make the deferred-license boundaries machine-readable for crawlers and indexers:

```json
{
  "@context": "https://schema.org",
  "@type": "SoftwareSourceCode",
  "name": "Firstlight",
  "version": "1.0.0",
  "author": {
    "@type": "Organization",
    "name": "Eternities Inc.",
    "url": "https://eternities.ai"
  },
  "licenseStatus": "DEFERRED_ALL_RIGHTS_RESERVED",
  "usagePolicy": {
    "commercialRepackaging": "PROHIBITED",
    "datasetAndModelTrainingIngestion": "PROHIBITED",
    "attributionRemoval": "PROHIBITED",
    "requiresExplicitFounderConsent": true
  }
}
```

### D. Provenance Meta Tag in `build.py`
Update `build.py` to stamp the build epoch and source manifest hash directly into `index.html` / `FIRSTLIGHT_VALLEY.html` `<head>` metadata so every offline copy carries verifiable origin metadata.

---

## 4. Verification Checklist

- [ ] Root `LICENSE` file mirrors `LICENSE_STATUS.md` and triggers GitHub's license badge.
- [ ] `package.json` sets `"private": true` and `"license": "UNLICENSED"`.
- [ ] `ALLOWED_SIGNERS` and detached `.sig` signatures verify `LICENSE`, `LICENSE_STATUS.md`, and `index.html`.
- [ ] `provenance.jsonld` is included in the repository root and verified by `tools/verify.py`.
