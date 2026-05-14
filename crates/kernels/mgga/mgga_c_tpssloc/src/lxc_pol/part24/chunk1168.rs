//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1168/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1168<F: Float>(t12168: F, t12178: F, t12238: F, t1336: F, t2013: F, t6987: F, t81122: F, t81125: F, t81127: F, t81132: F, t81140: F, t81147: F, t81149: F, t81154: F, t81157: F, t81160: F, t81165: F, t81169: F, t81173: F, t81177: F, t81181: F) -> (F,) {
    let t81183 = -0.24674011002723396548e-1 * t81122 + 0.12337005501361698274e-1 * t81125 + 0.11514538467937585055e0 * t81127 - 0.49348022005446793095e-1 * t81132 - t1336 * t6987 * t12178 - t1336 * t6987 * t12168 + t12238 * t2013 - 0.74022033008170189643e-1 * t81140 - t81147 - 0.24674011002723396547e-1 * t81149 + t81154 + 0.82246703342411321825e-2 * t81157 - 0.23029076935875170111e0 * t81160 - 0.14804406601634037928e0 * t81165 + 0.49348022005446793095e-1 * t81169 + 0.49348022005446793095e-1 * t81173 - 0.82246703342411321825e-2 * t81177 + 0.49348022005446793095e-1 * t81181;
    (t81183,)
}
