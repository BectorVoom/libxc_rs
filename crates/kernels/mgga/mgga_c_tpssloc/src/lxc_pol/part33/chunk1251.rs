//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1251/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1251<F: Float>(t1799: F, t26331: F, t26446: F, t96951: F, t107320: F, t107326: F, t107331: F, t107335: F, t107339: F, t107343: F, t107348: F, t1336: F, t26458: F, t6420: F, t81073: F, t81075: F, t90980: F, t97026: F, t97043: F, t97049: F, t97070: F, t97095: F, t97108: F) -> (F,) {
    let t107353 = t26331 * t26446 * t96951 * t1799;
    let t107356 = -3.0 * t1336 * t26458 * t6420 - t81073 - t81075 - 0.82246703342411321825e-2 * t107320 + 0.24674011002723396547e-1 * t97026 - 0.49348022005446793095e-1 * t97043 - 0.24674011002723396548e-1 * t97049 - 0.24674011002723396548e-1 * t107326 + 0.49348022005446793095e-1 * t97070 - 0.49348022005446793095e-1 * t107331 + 0.9869604401089358619e-1 * t107335 + 0.49348022005446793095e-1 * t107339 + 0.49348022005446793095e-1 * t107343 + 0.23029076935875170111e0 * t97095 - 0.49348022005446793095e-1 * t107348 + 0.24674011002723396547e-1 * t90980 + 0.14804406601634037928e0 * t107353 + 0.11514538467937585055e0 * t97108;
    (t107356,)
}
