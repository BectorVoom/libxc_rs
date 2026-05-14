//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 257/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk257<F: Float>(t265: F, t504: F, t1245: F, t1246: F, t1235: F, t493: F, t1201: F, t1244: F, t470: F, t494: F, t1241: F, t1191: F, t1236: F, t1238: F, t498: F, t500: F, t1096: F, t1121: F, t1161: F, t1163: F, t1168: F, t193: F, t336: F, t873: F) -> (F, F, F, F, F, F, F) {
    let t505 = t265 < t504;
    let t1247 = t1245 * t1246;
    let t1249 = t493 * t1235;
    let t1251 = t1201 * t494 + t1244 * t1247 + t1249 * t470;
    let t1252 = t1241 * t1251;
    let t1254 = t1191 * t498 + t1236 * t498 - t1238 * t1252;
    let t1256 = 1.0 / t500;
    let t1260 = piecewise3(t505, t1254 * t1256 * t193 * t336 - t1096 + t1121 + t1161 + t1163 - t1168, t873);
    (t1247, t1249, t1251, t1252, t1254, t1256, t1260)
}
