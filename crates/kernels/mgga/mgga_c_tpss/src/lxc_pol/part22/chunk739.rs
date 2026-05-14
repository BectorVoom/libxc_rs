//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 739/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk739<F: Float>(t4205: F, t4206: F, t1089: F, t1095: F, t1554: F, t1300: F, t924: F, t140: F, t1557: F, t1098: F, t3032: F, t926: F, t4047: F, t1100: F, t4052: F, t1101: F, t3431: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4207 = t4205 * t4206;
    let t4209 = 0.17315859105681463759e2 * t1089 * t4207;
    let t4210 = t1554 * t1095;
    let t4212 = t1300 * t924;
    let t4216 = t140 * t1557;
    let t4217 = t1098 * t4216;
    let t4219 = t926 * t3032;
    let t4220 = t4219 * t4047;
    let t4223 = t926 * t1100;
    let t4224 = t4223 * t4052;
    let t4227 = t1101 * t3431;
    (t4207, t4209, t4210, t4212, t4217, t4219, t4220, t4223, t4224, t4227)
}
