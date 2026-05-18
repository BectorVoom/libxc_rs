//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 667/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk667<F: Float>(t1080: F, t3001: F, t4205: F, t1089: F, t1095: F, t1554: F, t1300: F, t924: F) -> (F, F, F, F, F) {
    let t4206 = t3001 * t1080;
    let t4207 = t4205 * t4206;
    let t4209 = F::new(0.17315859105681463759e2) * t1089 * t4207;
    let t4210 = t1554 * t1095;
    let t4212 = t1300 * t924;
    (t4206, t4207, t4209, t4210, t4212)
}
