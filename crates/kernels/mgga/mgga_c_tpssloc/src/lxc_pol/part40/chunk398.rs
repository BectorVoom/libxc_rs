//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 398/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk398<F: Float>(t1287: F, t17: F, t1284: F, t182: F, t521: F, t67: F, t758: F, t172: F) -> (F, F, F, F, F) {
    let t1288 = t17 * t1287;
    let t1290 = F::new(0.19751673498613801407e-1) * t1284 * t182;
    let t1291 = t521 * t67;
    let t1293 = F::new(0.18311447306006545054e-3) * t1291 * t758;
    let t1294 = t521 * t172;
    (t1288, t1290, t1291, t1293, t1294)
}
