//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1221/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1221<F: Float>(t24684: F, t27634: F, t1210: F, t24654: F, t24721: F, t11168: F, t11809: F, t11855: F, t11863: F, t2121: F, t24664: F, t24670: F, t24736: F, t27636: F, t27638: F, t27644: F, t3448: F, t3493: F, t3503: F, t3531: F, t7339: F, t7345: F, t86228: F) -> (F,) {
    let t86234 = t27634 * t24684;
    let t86248 = t24721 * t1210 * t24654;
    let t86253 = -t24736 * t3531 / 384.0 - t7345 * t11809 / 384.0 + t86228 / 768.0 + t7339 * t11855 / 1536.0 - t7345 * t11863 / 384.0 - 0.60559134141210586284e-3 * t86234 * t24664 + 0.30279567070605293142e-3 * t86234 * t24670 + 0.60559134141210586284e-3 * t27636 * t3503 * t3493 * t27638 - 0.30279567070605293142e-3 * t27636 * t1210 * t3493 * t27644 + 0.30279567070605293142e-3 * t86248 - t2121 * t3448 * t11168 / 48.0;
    (t86253,)
}
