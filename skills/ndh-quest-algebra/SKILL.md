# Skill: `ndh-quest-algebra` — Deterministic 80D Symplectic Narrative & World State

**Layer:** Core Skill (`skills/`) · Engine-Agnostic · Zero-Float `Q16.16` Fixed-Point  
**Dependencies:** Pairs with `game-loop`, `game-narrative`, `game-save`, `game-measure`  
**Methodology:** `NDH-Meta-Systems` (Sealed-Layer Neutral · Non-Conscious Simulation Engine)  

---

## 1. Purpose & When to Load This Skill
Load `ndh-quest-algebra` whenever a game requires **branching narrative state, faction diplomacy, NPC trust tracking, or world-rule mutations** that must remain **100% deterministic** across desktop, mobile ARM64, and WebAssembly (`WASM`) targets without ad-hoc boolean flag spaghetti.

---

## 2. Core Invariants (Non-Negotiable)
1. **Zero Floating-Point Drift (`Law 2` & `Law 7`):** Never store narrative, faction, or quest trajectories in `Float32`/`Float64`. Use signed `Q16.16` fixed-point `Int32` scalars (`1.0 == 65536`, `0.5 == 32768`) via `apply_quest_operator_80d_q16`.
2. **Epistemic Honesty (`game-ai` & `game-narrative`):** Quest Algebra operators and NPC affinity vectors are **deterministic matrix math**, never conscious agents. Keep poetic or mythic world themes strictly inside player-facing text (`Ledger 2`), never inside engine trust logic.
3. **Two-Ledger Binding (`game-narrative`):**
   * **Channels `0..20` (`WORLD_RULES`):** Faction dominance & resource levels $\to$ **Ledger 1 (Objective World Truth)**.
   * **Channels `20..40` (`CHAR_AFFINITY`):** NPC trust & perceived standing $\to$ **Ledger 2 (Relational / Perceived State)**.
   * **Channels `40..60` (`QUEST_COORDS`, $q$):** Active story branch coordinates.
   * **Channels `60..80` (`ENV_TENSION`, $p$):** Narrative pacing and environmental momentum.

---

## 3. Fixed-Timestep Integration Pattern
Invoke discrete $80 \times 80$ `Int8` operator transitions **only on explicit narrative events** inside the fixed-timestep simulation update—never inside variable-rate render frames:

```rust
// Example: Applying a discrete FACTION_BETRAYAL transition inside fixed update()
use ndh_core::quest_algebra_q16::{apply_quest_operator_80d_q16, Q16, DIM_80, MATRIX_LEN};

pub fn dispatch_narrative_event(
    operator_matrix: &[i8; MATRIX_LEN],
    current_state: &[Q16; DIM_80],
    next_state: &mut [Q16; DIM_80],
) {
    unsafe {
        apply_quest_operator_80d_q16(
            operator_matrix.as_ptr(),
            current_state.as_ptr(),
            next_state.as_mut_ptr(),
        );
    }
}
```

---

## 4. Save Schema & Headless Verification (`Law 5` & `game-measure`)
* **Versioned Save Payload (`game-save`):** Serialize `[Q16; 80]` as 320 raw little-endian bytes (`80 * 4 bytes`) prefixed with a `u32` schema version (`0x0001`) and `SHA-256` checksum.
* **Headless Probe (`game-measure`):** Verify every narrative operator with an offline unit test asserting exact integer outputs before wiring UI or dialogue trees.
