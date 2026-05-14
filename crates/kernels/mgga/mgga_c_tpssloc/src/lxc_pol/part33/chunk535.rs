//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 535/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk535<F: Float>(t1222: F, t1731: F, t1744: F, t1229: F, t3247: F, t3242: F, t3584: F, t1653: F, t248: F, t3521: F, t1227: F, t1735: F, t3570: F, t1213: F, t1009: F, t1720: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4957 = t1731 * t1222;
    let t4959 = t1744 * t1222;
    let t4972 = t1229 * t3247;
    let t4987 = t3584 * t3242;
    let t4993 = t248 * t3521 * t1653;
    let t4994 = t1227 * t4993;
    let t4997 = t248 * t3570 * t1735;
    let t4998 = t1213 * t4997;
    let t5000 = t1720 * t1009;
    (t4957, t4959, t4972, t4987, t4993, t4994, t4997, t4998, t5000)
}
