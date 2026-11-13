//! Discrete Non-Abelian Holonomy Loop Tracker (`#![no_std]`, Zero Floats)
//! Computes path-ordered parallel transport around closed narrative/world loops
//! (`Hol_∇(γ)`) in exact Q32.32 fixed-point arithmetic so Soft-Worlds retain
//! topological path memory without unbounded history logs.
//! Methodology: NDH-Meta-Systems (Sealed-Layer Neutral)

#![no_std]

use crate::symplectic_verlet_q32::{q32_mul, Q32, Q32_ONE, Q32_ZERO};

/// 4x4 gauge connection transport matrix in Q32.32 fixed-point arithmetic,
/// representing parallel transport across the 4 narrative/world strata:
/// [0: World Rules | 1: Character Affinity | 2: Quest Position | 3: Environmental Tension]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConnectionTransport4 {
    pub m: [[Q32; 4]; 4],
}

impl ConnectionTransport4 {
    /// Identity holonomy element (`Hol(γ) == I_4` => flat / path-independent loop).
    pub const IDENTITY: Self = Self {
        m: [
            [Q32_ONE, Q32_ZERO, Q32_ZERO, Q32_ZERO],
            [Q32_ZERO, Q32_ONE, Q32_ZERO, Q32_ZERO],
            [Q32_ZERO, Q32_ZERO, Q32_ONE, Q32_ZERO],
            [Q32_ZERO, Q32_ZERO, Q32_ZERO, Q32_ONE],
        ],
    };

    /// Path-ordered matrix composition `self * step` (`U_{n+1} = U_step * U_n`).
    pub fn compose(&self, step: &Self) -> Self {
        let mut out = [[Q32_ZERO; 4]; 4];
        let mut r = 0;
        while r < 4 {
            let mut c = 0;
            while c < 4 {
                let mut acc: Q32 = 0;
                let mut k = 0;
                while k < 4 {
                    acc = acc.saturating_add(q32_mul(step.m[r][k], self.m[k][c]));
                    k += 1;
                }
                out[r][c] = acc;
                c += 1;
            }
            r += 1;
        }
        Self { m: out }
    }

    /// Computes exact L1 curvature deviation `||Hol(γ) - I_4||_1` in Q32.32.
    /// Zero indicates a flat trivial loop; non-zero measures accumulated topological memory.
    pub fn curvature_residue_l1(&self) -> Q32 {
        let mut sum: Q32 = 0;
        let mut r = 0;
        while r < 4 {
            let mut c = 0;
            while c < 4 {
                let target = if r == c { Q32_ONE } else { Q32_ZERO };
                let diff = (self.m[r][c] - target).abs();
                sum = sum.saturating_add(diff);
                c += 1;
            }
            r += 1;
        }
        sum
    }
}

/// Tracks closed-loop holonomy across discrete manifold waypoints (`RegionId`).
pub struct HolonomyLoopTracker {
    pub origin_region: u32,
    pub current_region: u32,
    pub step_count: u32,
    pub transport: ConnectionTransport4,
}

impl HolonomyLoopTracker {
    pub const fn new(origin_region: u32) -> Self {
        Self {
            origin_region,
            current_region: origin_region,
            step_count: 0,
            transport: ConnectionTransport4::IDENTITY,
        }
    }

    /// Advances parallel transport along an edge `current_region -> next_region`
    /// with local connection curvature `edge_transport`.
    pub fn step_edge(&mut self, next_region: u32, edge_transport: &ConnectionTransport4) {
        self.transport = self.transport.compose(edge_transport);
        self.current_region = next_region;
        self.step_count = self.step_count.saturating_add(1);
    }

    /// Returns `Some(curvature_residue_l1)` when a non-trivial closed loop (`step_count > 0`)
    /// returns to `origin_region`.
    pub fn evaluate_closed_loop(&self) -> Option<Q32> {
        if self.step_count > 0 && self.current_region == self.origin_region {
            Some(self.transport.curvature_residue_l1())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_loop_accumulates_non_trivial_holonomy_residue() {
        let mut tracker = HolonomyLoopTracker::new(100); // Start at Region 100

        // Edge 100 -> 200: introduces cross-coupling between World Rules (0) and Affinity (1)
        let mut edge_a = ConnectionTransport4::IDENTITY;
        edge_a.m = Q32_ONE;

        // Edge 200 -> 300: introduces coupling between Affinity (1) and Tension (3)
        let mut edge_b = ConnectionTransport4::IDENTITY;
        edge_b.m = Q32_ONE;

        // Edge 300 -> 100: returns to origin Region 100 with flat return connection
        let edge_c = ConnectionTransport4::IDENTITY;

        tracker.step_edge(200, &edge_a);
        assert_eq!(tracker.evaluate_closed_loop(), None);

        tracker.step_edge(300, &edge_b);
        assert_eq!(tracker.evaluate_closed_loop(), None);

        tracker.step_edge(100, &edge_c);

        // Returning to Region 100 closes the loop with non-zero holonomy residue (3.0 in Q32.32)
        let residue = tracker.evaluate_closed_loop().expect("loop is closed");
        assert_eq!(residue, 3 * Q32_ONE);
    }
}
