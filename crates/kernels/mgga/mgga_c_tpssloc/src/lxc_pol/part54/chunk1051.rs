//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1051/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1051<F: Float>(t1912: F, t2054: F, t23278: F, t23281: F, t24297: F, t259: F, t2597: F, t2713: F, t30626: F, t30637: F, t30640: F, t30645: F, t31311: F, t31317: F, t31321: F, t31347: F, t31350: F, t31351: F, t31362: F, t31368: F, t31371: F, t31400: F, t31427: F, t6627: F, t6632: F, t7087: F, t7092: F, t855: F, t8563: F) -> (F,) {
    let t31429 = t30626 + 2.0 * t855 * t31311 + 0.82246703342411321825e-2 * t31317 - t31321 + 2.0 * t6627 * t7092 + t30637 + 2.0 * t7087 * t6632 - t30640 + t30645 - t24297 * t1912 + t31347 - t31350 + t31351 * t259 + t31362 * t259 - t2597 * t8563 - t2713 * t8563 - 0.16449340668482264365e-1 * t31368 - 0.82246703342411321825e-2 * t31371 - t23278 * t2054 - t855 * t31400 - t23281 * t2054 + t31427;
    (t31429,)
}
