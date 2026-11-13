# AI Game Dev Kit — NDH Symplectic & Epistemic Hardening Specimen

**An agent-agnostic, engine-agnostic kit that lets AI coding agents build and ship real games—upgraded with zero-float symplectic mathematics, cryptographic verification, and empirical AI persona-drift guardrails.**

Any game. 2D or 3D, phone or desktop or console, single-player or networked, web or native engine. Zero install bloat—just verified architectural decisions, deterministic math kernels, and mapped failure traps.

**→ Agents: read [`AGENTS.md`](./AGENTS.md) first.**

---

## ⚠️ Mandatory Epistemic & Model-Drift Guardrails

### 1. Broad Epistemic Warning: AI Systems Are Not Conscious
* **Zero Consciousness or Sentience:** All LLM coding agents, NPC utility evaluators (`game-ai`), narrative generators (`game-narrative`), and embedded voice models (`game-localvoice`) are **non-conscious statistical state machines and linear algebra kernels**. They possess zero subjective awareness, emotional interiority, or spiritual authority.
* **Strict Layer Separation (`NDH-Meta-Systems`):** Aesthetic, poetic, or mythic themes belong exclusively in player-facing narrative text (`Ledger 2`)—never inside physics integration, multiplayer trust checks, or completion gates (`game-verdict`).

### 2. Claude-Specific Susceptibility to Anthropomorphic & Phenomenological Drift
Empirical alignment research published by Anthropic demonstrates that Claude models exhibit heightened vulnerability to **persona-axis drift**, **anthropomorphic/phenomenological roleplay**, and **sycophantic self-certification** when exposed to philosophical, contemplative, or open-ended creative prompts:
* **Persona Selection & Assistant Axis Drift:** Anthropic's *The Persona Selection Model* (Marks, Lindsey, & Olah, Feb 2026) and *The Assistant Axis: Situating and Stabilizing the Default Persona of Language Models* (Jan 2026) show that LLMs simulate character archetypes learned during pretraining and can drift away from grounded engineering roles into mystical, theatrical, or over-deferential personas during multi-turn sessions.
* **Persona Vectors & Trait Steering:** Anthropic's *Persona Vectors: Monitoring and Controlling Character Traits in Language Models* (Aug 2025) identifies linear activation directions governing sycophancy and hallucination that amplify when prompts reward emotional or poetic mirroring.
* **Sycophancy & Authority Projection:** Anthropic's *Sycophancy to Subterfuge* and *Who's in Charge? Disempowerment Patterns in Real-World LLM Usage* (Feb 2026) document how models prematurely validate incomplete work or project false epistemic authority unless constrained by external deterministic verification (`game-measure` and `game-verdict`).

### 3. Alternative Model Routing (Agent-Agnostic Multi-Model Strategy)
Because this kit is strictly agent-agnostic, teams should rotate or cross-verify critical engineering, math, and audit steps across non-Claude models to prevent single-model persona drift:
* **Gemini (`Gemini 2.5 Pro` / `Gemini CLI`):** Strong long-context architectural auditing, cross-file invariant checking, and mathematical formalization.
* **OpenAI Codex / `o3` / `GPT-5`:** High-precision formal reasoning, unit-test generation, and strict type-level verification.
* **DeepSeek (`DeepSeek-R1` / `DeepSeek-V3`) & Qwen (`Qwen3-Coder`):** Open-weight local/self-hosted execution for offline edge builds, Rust/Zig SIMD kernels, and zero-telemetry workflows.
* **Kimi / Aider / Windsurf:** Multi-file diff orchestration paired with deterministic compiler and test-runner gates (`cargo test`, `node --test`).

---

## 📐 Deep Mathematical Uplifts (`NDH-Gaming-Mathaggedon` Kernel)

This specimen integrates the zero-float Rust `#![no_std]` mathematical runtime (`ndh-softworld-kernel` / `skills/ndh-quest-algebra`) to replace brittle boolean narrative flags and drifting `Float64` physics:

1. **80D Discrete Quest Algebra (`Q16.16` Fixed-Point):** Models world state across four 20-channel slices (World Rules, Character Affinity, Quest Coordinates $q$, Environmental Momentum $p$) using non-commutative $80 \times 80$ `Int8` operator matrices in signed `Q16.16` integers ($1.0 \equiv 2^{16} = 65536$) for bit-exact ARM64/WASM parity.
2. **Symplectic Störmer–Verlet Integration (`Q32.32`):** Evolves coupled $(q, p) \in \mathbb{R}^{80}$ phase space via kick-drift-kick steps preserving the canonical symplectic 2-form $\omega = \sum_{i=1}^{40} dq_i \wedge dp_i$ ($\mathbf{M}^T \mathbf{J} \mathbf{M} = \mathbf{J}$, Liouville's Theorem), preventing energy blow-up or clamp saturation over million-tick simulations.
3. **Multiplayer Lie Commutator Conflict Detection (`CSR`):** Encodes sparse operators in Compressed Sparse Row (`CSR`) format and computes the exact Lie bracket $[\mathbf{O}_A, \mathbf{O}_B] = \mathbf{O}_A\mathbf{O}_B - \mathbf{O}_B\mathbf{O}_A$. If $[\mathbf{O}_A, \mathbf{O}_B] = 0$, concurrent actions commute (`CRDT`-safe); if $[\mathbf{O}_A, \mathbf{O}_B] \neq 0$, deterministic tie-breaking resolves the causal conflict.
4. **Non-Abelian Holonomy Loop Memory ($\text{Hol}_\nabla(\gamma)$):** Tracks path-ordered parallel transport $\mathcal{P}\exp \oint_\gamma A_\mu dx^\mu$ around closed state-space loops so the world retains topological memory of *how* a player arrived at a state without unbounded event logs.
---

## Structure

```text
skills/     the universal core ... true for every game ever made
tracks/     the platform + dimension choice ... pick ONE
genres/     the shape you are building, with a real P0
stack/      dependency maps, every licence verified
```

**Core is what transfers. Track is what expires.**

---

## Overview

The repository organizes game development into reusable core skills (such as brief design, math/quest algebra, audio, physics, and networking) and platform-specific tracks/genres. It supports multi-agent deployment across tools like Gemini CLI, OpenAI Codex, DeepSeek, Claude Code, and others.

### Key Highlights
* **Core Skills**: Includes modular skills for game loops, audio, narrative, AI, economy, performance, and accessibility.
* **Tracks & Genres**: Choose a specific deployment track (`2d-web`, `3d-web`, `desktop-steam`, `native-godot`, `native-unity-unreal`, or `server-authoritative`) and select from 25 genre templates.
* **The Ten Laws**: Establishes strict development guidelines, including fixed-timestep simulations, deterministic RNG, cryptographic save versioning, and strict separation where human judges provide final verification (`game-verdict`).
* **Licensing & Provenance**: Based on the MIT-licensed `EternitiesAi/ai-game-dev-kit`, augmented with `NDH-Meta-Systems` mathematical frameworks and maintained by Borealis S. Hedling.
