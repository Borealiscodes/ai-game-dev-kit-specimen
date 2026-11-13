//! NDH-GAMING-MATHAGGEDON — Unified Soft-World Mathematical Runtime (`refactor/src/lib.rs`)
//!
//! Zero-float, `#![no_std]` deterministic geometric engine unifying:
//! 1. `quest_algebra_q16`     — 80x80 discrete non-commutative Quest Algebra + C-ABI FFI.
//! 2. `symplectic_verlet_q32` — Exact Q32.32 Störmer–Verlet phase-space integrator (Liouville-preserving).
//! 3. `lie_commutator_csr`    — Sparse CSR 80x80 operators + Lie bracket `[A, B]` conflict detector.
//! 4. `holonomy_tracker`      — Discrete closed-loop parallel transport (`Hol_∇(γ)`) path memory.
//!
//! Methodology: NDH-Meta-Systems (Sealed-Layer Neutral · Non-Conscious Simulation Engine)
//! Canonical Reference: DOI 10.5281/zenodo.24149283

#![no_std]

pub mod holonomy_tracker;
pub mod lie_commutator_csr;
pub mod quest_algebra_q16;
pub mod symplectic_verlet_q32;

pub use holonomy_tracker::{ConnectionTransport4, HolonomyLoopTracker};
pub use lie_commutator_csr::{operators_commute, CsrError, SparseOperator80};
pub use quest_algebra_q16::{
    apply_quest_operator_80d_q16, apply_quest_operator_80d_safe, SymplecticStateQ16, DIM_80,
    MATRIX_LEN, Q16, Q16_HALF, Q16_ONE, Q16_ZERO,
};
pub use symplectic_verlet_q32::{
    conservative_force_q32, q32_mul, step_symplectic_verlet_q32, SymplecticPhaseQ32, PHASE_DIM,
    Q32, Q32_HALF, Q32_ONE, Q32_ZERO,
};
