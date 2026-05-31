//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 840/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk840<F: Float>(t1824: F, t6387: F, t12250: F, t1343: F, t820: F, t3792: F, t119: F, t20416: F, t210: F, t12291: F, t12330: F, t12335: F, t1315: F, t16341: F, t16350: F, t19915: F, t19917: F, t19933: F, t3790: F, t5235: F, t6417: F) -> (F, F, F, F, F, F, F) {
    let t20489 = t6387 * t1824;
    let t20490 = t20489 * t12250;
    let t20492 = t1343 * t820 * t20490;
    let t20495 = t20489 * t3792;
    let t20497 = t1343 * t820 * t20495;
    let t20500 = t119 * t20416;
    let t20501 = t210 * t20500;
    let t20508 = -F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t16341 - t5235 * t6417 / F::cast_from(1024.0_f64) - t12291 * t20492 / F::cast_from(512.0_f64) + t3790 * t20497 / F::cast_from(512.0_f64) - t1315 * t20501 / F::cast_from(48.0_f64) + F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t16350 - t12330 - t12335 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t19915 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t19917 + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t19933;
    (t20489, t20490, t20492, t20495, t20497, t20501, t20508)
}
