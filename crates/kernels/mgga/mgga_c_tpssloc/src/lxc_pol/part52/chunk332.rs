//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 332/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk332<F: Float>(t1284: F, t184: F, t17: F, t521: F, t750: F, t182: F, t67: F, t758: F, t172: F, t763: F, t532: F, t571: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1285 = t1284 * t184;
    let t1286 = t17 * t1285;
    let t1287 = t521 * t750;
    let t1288 = t17 * t1287;
    let t1290 = F::cast_from(0.19751673498613801407e-1_f64) * t1284 * t182;
    let t1291 = t521 * t67;
    let t1293 = F::cast_from(0.18311447306006545054e-3_f64) * t1291 * t758;
    let t1294 = t521 * t172;
    let t1296 = F::cast_from(0.5848223622634646207e0_f64) * t1294 * t763;
    let t1297 = t532 * t571;
    (t1285, t1286, t1287, t1288, t1290, t1291, t1293, t1294, t1296, t1297)
}
