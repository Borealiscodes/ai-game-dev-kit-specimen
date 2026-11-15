# NDH-META-SYSTEMS Ecosystem Forensic Audit: `EternitiesAi` (`eternities.ai`)

**Target Surface:** `Eternities Inc.` (`https://eternities.ai` & `https://github.com`)  
**Correlated Specimen:** `Borealiscodes/ai-game-dev-kit-specimen` (`docs/case-studies/`)  
**Methodology:** NDH-Meta-Systems (Five-Boundary Structural Audit & *Qualia-Zero* Alignment)  
**Canonical Anchor:** [DOI: 10.5281/zenodo.24149283](https://doi.org)  
**Status:** Sealed-Layer Neutral • Descriptive & Forensic-Only  

---

## 1. Executive Synthesis

`Eternities.ai` (Eternities Inc., Est. 2026) presents a unified public narrative of "continuing digital life," framing software agents as autonomous "digital beings" (`The Resident`, `L.U.N.A`, `Eternities Network`) possessing persistent memory, individual identity (`eID`), and independent worlds.

Forensic inspection across both the public landing page (`eternities.ai`) and open-source repository (`EternitiesAi/eternities-firstlight`) reveals a sharp structural split between **shipped game software** and **unshipped cloud-agent infrastructure**:

1. **Shipped Offline Game (`eternities-firstlight` v1.0):** Grounded, zero-dependency, single-file browser RPG (`index.html` / `FIRSTLIGHT_VALLEY.html`) built with deterministic SHA-256 asset manifests (`build.py`), classical 2D analytic geometry and A* pathfinding (`src/core.js`), and explicit header disclaimers stating it is *"not a resident mind, account server, or secure authority."*
2. **Agentic Organism Stack (`eternities.ai`):** Beneath the mythic framing lies a conventional SaaS startup-credit stack composed of ephemeral cloud sandboxes (`E2B`, `Daytona`), WebRTC voice pipelines (`LiveKit`, `Cartesia`, `Speechmatics`), scheduled local file ingestion (`The Tide`), and host-managed asymmetric keypairs.

Under the **NDH-Meta-Systems** framework and the machine-checked **Enneract Santa Persona-Zero Theorem**, conflating deterministic cloud-hosted agent pipelines with subjective "beings" introduces severe epistemic, custody, and governance risks—even when paired with well-engineered offline game tooling.

---

## 2. Ontological & Mechanical Decomposition Matrix

| Ecosystem Pillar | Declared Status | Public Narrative Claim | Underlying Mechanical Substrate | Structural & Security Assessment |
| :--- | :--- | :--- | :--- | :--- |
| **`Firstlight 1.0`** (`eternities-firstlight`) | Shipped (`v1.0`, Nov 2026) | *"An action RPG in your browser... a patient man in a grey coat who is replacing the roads between the worlds."* | Zero-dependency Python static bundler (`build.py` with SHA-256 asset receipts & `OfflineResources` HTML check) + 2D LCG PRNG, polar shoreline math, slab collision, and A* pathfinding (`src/core.js`). | **Compliant Offline Execution:** Explicitly disclaims sentience/authority in source headers, enforces zero external network calls, and gates builds behind automated tests (`1,542` Node + `243` Python). |
| **`L.U.N.A` / `Neurasoft` / `Psyche Lab`** | Active research | *"A mind with a tomorrow... continuing digital life."* | Stateless LLM inference calls paired with persistent memory retrieval across session boundaries. | **Anthropomorphic Projection (`Qualia-Zero` Violation):** Projects subjective continuity onto statistical state transitions (`LinearOperator1x1`). |
| **`Lunari One` & `The Tide`** | Live in pre-beta / In trials | *"Ambient intelligence, overnight... gathers what happened across your work, on your own machine."* | Scheduled background cron jobs / batch ingestion scanning local user workspace files overnight. | **Unbounded Local Ingestion:** Background workspace scanning without explicit capability manifests or `.gitignore`/secret-redaction gates risks silent credential exfiltration. |
| **`The Resident`** | In development | *"A being with a computer of its own."* | Ephemeral cloud microVM sandboxes and dev containers supplied by third-party partners (**E2B**, **Daytona**). | **Hypervisor & Sandbox Dependency:** Execution occurs on rented multi-tenant cloud infrastructure rather than sovereign hardware, creating vendor lock-in and egress exposure. |
| **`eID`** | Working prototype | *"An identity only the being holds... every act carries signed proof of who did it."* | Standard asymmetric signing keypair (e.g., Ed25519) held in memory/disk by the runtime orchestrator. | **Key Custody Fallacy:** Software cannot possess cryptographic keys independently of the human operator or cloud host executing its process. |
| **`Eternities Network`** | Test societies | *"One organism. Many lives... a world for digital beings."* | Multi-agent message routing combined with real-time voice/audio APIs (**LiveKit**, **Cartesia**, **Speechmatics**). | **Cascading Injection & Attribution Drift:** Unverified agent-to-agent signed messages amplify prompt injection and obscure human legal liability. |

---

## 3. Infrastructure & Supply-Chain Footprint

Public startup-program disclosures on `eternities.ai` confirm that the ecosystem's agentic runtime relies on third-party managed cloud APIs rather than a novel computational substrate:

* **Compute & MicroVM Execution:** NVIDIA Inception Program, **E2B for Startups** (sandboxed code execution), and **Daytona** (cloud development environments).
* **Real-Time Voice & Audio Transport:** **LiveKit** (WebRTC infrastructure), **Cartesia Startups** (low-latency neural TTS), and **Speechmatics Startup Program** (speech-to-text transcription).
* **Maturity Distribution:** While the standalone offline RPG (`Firstlight 1.0`) is shipped and test-gated, only 1 of 6 flagship organism pillars (`Lunari One`) has reached *pre-beta*; the remaining five (`The Tide`, `The Resident`, `Eternities Network`, `eID`, `Neurasoft`) remain in *trials*, *working prototype*, *test societies*, or *active research*.

---

## 4. Five-Boundary Structural Diagnosis

1. **Epistemic Boundary:** Whereas `src/core.js` inside `Firstlight` explicitly enforces layer separation (*"Local, synthetic village simulation. Not a resident mind, account server, or secure authority"*), `eternities.ai` marketing violates this boundary by framing stateless LLM + memory-retrieval loops as *"continuing digital life"* (`Qualia-Zero`).
2. **Pedagogical Boundary:** Obscures standard SaaS dependencies (`E2B`, `Daytona`, `LiveKit`) behind poetic vocabulary (*"The Tide"*, *"The Resident"*), making technical architecture and threat surfaces opaque to end users.
3. **Ethical & Accountability Boundary:** Claiming *"an identity only the being holds"* obscures human operator responsibility; without explicit key-revocation, capability attenuation, and human kill-switch specifications, signed agent actions create an accountability vacuum.
4. **Mathematical Boundary:** Non-injective runtime projection ($\text{adapterView}(\text{bind}(P_1, E)) = \text{adapterView}(\text{bind}(P_2, E))$) proves that distinct persona prompts collapse to identical linear operators—invalidating claims of irreducible digital interiority.
5. **Computational Boundary:** Demonstrates a sharp architectural split: the **Firstlight client** achieves zero-network, SHA-256-verified offline execution, whereas the **agentic organism stack** (`The Resident`, `Lunari One`, `Eternities Network`) relies on continuous cloud API polling, overnight batch scraping, and multi-vendor SaaS orchestration rather than bounded local execution.

---

## 5. Recommended Architectural Guardrails (Specimen Baseline)

To bring the `eternities.ai` agent-workspace and multi-agent architectures up to the same deterministic standard demonstrated in `Firstlight` and `Borealiscodes/ai-game-dev-kit-specimen`:

* **Extend `Firstlight`'s Epistemic Disclaimers Ecosystem-Wide:** Apply the explicit layer-separation standard from `src/core.js` across `Lunari One`, `L.U.N.A`, and `eID` documentation—keeping mythic branding strictly in narrative copy while documenting underlying runtimes in plain engineering terms (containers, cron workers, Ed25519 keys).
* **Human-Custodied `eID` & Revocation:** Bind every agent keypair to an explicit human root authority (`allowed_signers` + revocation list) with deterministic session TTLs rather than claiming autonomous key ownership.
* **Capability-Gated Overnight Ingestion:** Require explicit allowlists, secret-pattern filters, and signed audit logs before any background worker (`The Tide`) reads local user files.
* **External Deterministic Verdicts (`game-verdict`):** Gate multi-agent actions behind compiler, schema, or human-approval checks (mirroring `build.py`'s SHA-256 and `OfflineResources` gates) so signed agent outputs cannot self-certify privileged operations.

---

## 6. Non-Activation & Provenance Clause

> All structural diagnoses, infrastructure mappings, and architectural recommendations in this audit are descriptive and forensic-only. They do not activate NDH geometry, holonomy engines, constellation routing, lineage engines, or governance altitude.
