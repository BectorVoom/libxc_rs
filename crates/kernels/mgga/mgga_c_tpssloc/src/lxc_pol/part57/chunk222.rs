//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 222/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk222<F: Float>(t522: F, t588: F, t592: F, t521: F, t750: F, t17: F, t67: F, t758: F, t172: F, t763: F, t532: F, t571: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1274 = F::new(4.0) * t588 * t522;
    let t1276 = F::new(4.0) * t592 * t522;
    let t1287 = t521 * t750;
    let t1288 = t17 * t1287;
    let t1291 = t521 * t67;
    let t1293 = F::new(0.18311447306006545054e-3) * t1291 * t758;
    let t1294 = t521 * t172;
    let t1296 = F::new(0.5848223622634646207e0) * t1294 * t763;
    let t1297 = t532 * t571;
    (t1274, t1276, t1287, t1288, t1291, t1293, t1294, t1296, t1297)
}
