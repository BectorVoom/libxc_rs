//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 602/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk602<F: Float>(t6553: F, t8547: F, t1880: F, t1911: F, t2053: F, t2718: F, t1894: F, t2047: F, t214: F, t235: F, t8543: F, t226: F, t8359: F, t858: F, t1912: F, t2054: F, t259: F, t6627: F, t7087: F, t8334: F, t8338: F, t8539: F, t8544: F, t855: F) -> (F, F, F, F, F, F, F, F) {
    let t8548 = t6553 * t8547;
    let t8549 = t1880 * t8548;
    let t8553 = t2718 * t2053 * t1911;
    let t8556 = t1894 * t2047;
    let t8557 = t214 * t8556;
    let t8558 = t1880 * t8557;
    let t8560 = t235 * t8543;
    let t8562 = t8359 + 0.82246703342411321825e-2 * t8558 + t226 * t8560;
    let t8563 = t858 * t8562;
    let t8565 = t8334 - t8338 + 0.82246703342411321825e-2 * t8539 + t8544 * t259 - t7087 * t1912 - 0.82246703342411321825e-2 * t8549 - t6627 * t2054 + 2.0 * t855 * t8553 - t855 * t8563;
    (t8548, t8553, t8556, t8557, t8560, t8562, t8563, t8565)
}
