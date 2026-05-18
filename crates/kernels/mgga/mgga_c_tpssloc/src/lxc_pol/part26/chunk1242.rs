//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1242/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1242<F: Float>(t1352: F, t22633: F, t6976: F, t81052: F, t1992: F, t22897: F, t3792: F, t81094: F, t40475: F, t550: F, t81028: F, t12168: F, t12178: F, t12238: F, t1336: F, t2013: F, t6987: F, t81122: F, t81125: F, t81127: F, t81132: F, t81140: F, t81147: F, t81149: F, t81154: F, t81157: F, t81160: F, t81165: F) -> F {
    let t81169 = t22633 * t6976 * t81052 * t1352;
    let t81173 = t1992 * t22897 * t81094 * t3792;
    let t81177 = t1992 * t6976 * t40475 * t550;
    let t81181 = t1992 * t22897 * t81028 * t3792;
    let t81183 = -F::new(0.24674011002723396548e-1) * t81122 + F::new(0.12337005501361698274e-1) * t81125 + F::new(0.11514538467937585055e0) * t81127 - F::new(0.49348022005446793095e-1) * t81132 - t1336 * t6987 * t12178 - t1336 * t6987 * t12168 + t12238 * t2013 - F::new(0.74022033008170189643e-1) * t81140 - t81147 - F::new(0.24674011002723396547e-1) * t81149 + t81154 + F::new(0.82246703342411321825e-2) * t81157 - F::new(0.23029076935875170111e0) * t81160 - F::new(0.14804406601634037928e0) * t81165 + F::new(0.49348022005446793095e-1) * t81169 + F::new(0.49348022005446793095e-1) * t81173 - F::new(0.82246703342411321825e-2) * t81177 + F::new(0.49348022005446793095e-1) * t81181;
    t81183
}
