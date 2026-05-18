//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1126/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1126<F: Float>(t23047: F, t812: F, t1878: F, t244: F, t2230: F, t6589: F, t213: F, t229: F, t6546: F, t243: F, t598: F, t6584: F, t6604: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23048 = t812 * t23047;
    let t23056 = t1878 * t244;
    let t23061 = t2230 * t6589;
    let t23062 = t23061 * t213;
    let t23069 = t6546 * t229;
    let t23075 = t243 * t243;
    let t23076 = F::new(1.0) / t23075;
    let t23077 = t598 * t23076;
    let t23078 = t23077 * t213;
    let t23083 = t6584 * t6604;
    (t23048, t23056, t23061, t23062, t23069, t23075, t23076, t23077, t23078, t23083)
}
