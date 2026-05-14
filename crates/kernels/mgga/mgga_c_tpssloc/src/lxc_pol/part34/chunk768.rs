//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 768/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk768<F: Float>(t1824: F, t6387: F, t12250: F, t1343: F, t820: F, t3792: F, t119: F, t20416: F, t210: F, t12291: F, t12330: F, t12335: F, t1315: F, t16341: F, t16350: F, t19915: F, t19917: F, t19933: F, t3790: F, t5235: F, t6417: F) -> (F, F, F, F, F, F, F) {
    let t20489 = t6387 * t1824;
    let t20490 = t20489 * t12250;
    let t20492 = t1343 * t820 * t20490;
    let t20495 = t20489 * t3792;
    let t20497 = t1343 * t820 * t20495;
    let t20500 = t119 * t20416;
    let t20501 = t210 * t20500;
    let t20508 = -35.0 / 72.0 * t16341 - t5235 * t6417 / 1024.0 - t12291 * t20492 / 512.0 + t3790 * t20497 / 512.0 - t1315 * t20501 / 48.0 + 119.0 / 4608.0 * t16350 - t12330 - t12335 + 7.0 / 1536.0 * t19915 + 7.0 / 1536.0 * t19917 + 7.0 / 192.0 * t19933;
    (t20489, t20490, t20492, t20495, t20497, t20501, t20508)
}
