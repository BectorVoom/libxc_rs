//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 261/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk261<F: Float>(t25: F, t28: F, t1284: F, t184: F, t17: F, t521: F, t750: F, t182: F, t67: F, t758: F, t172: F, t763: F, t532: F, t571: F, t514: F, t606: F, t517: F, t1081: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t1285 = t1284 * t184;
    let t1286 = t17 * t1285;
    let t1287 = t521 * t750;
    let t1288 = t17 * t1287;
    let t1290 = 0.19751673498613801407e-1 * t1284 * t182;
    let t1291 = t521 * t67;
    let t1293 = 0.18311447306006545054e-3 * t1291 * t758;
    let t1294 = t521 * t172;
    let t1296 = 0.5848223622634646207e0 * t1294 * t763;
    let t1297 = t532 * t571;
    let t1298 = 1.0 / t514;
    let t1301 = piecewise3(t26, 0.0, 2.0 / 3.0 * t1298 * t606);
    let t1302 = 1.0 / t517;
    let t1305 = piecewise3(t29, 0.0, 2.0 / 3.0 * t1302 * t1081);
    (t1285, t1286, t1287, t1288, t1290, t1291, t1293, t1294, t1296, t1297, t1298, t1301, t1302, t1305)
}
