//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 774/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk774<F: Float>(t1089: F, t4202: F, t1542: F, t2998: F, t1080: F, t3001: F, t1095: F, t1554: F, t1300: F, t924: F, t140: F, t1557: F) -> (F, F, F, F, F, F, F, F) {
    let t4204 = F::new(0.5848223622634646207e0) * t1089 * t4202;
    let t4205 = t2998 * t1542;
    let t4206 = t3001 * t1080;
    let t4207 = t4205 * t4206;
    let t4209 = F::new(0.17315859105681463759e2) * t1089 * t4207;
    let t4210 = t1554 * t1095;
    let t4212 = t1300 * t924;
    let t4216 = t140 * t1557;
    (t4204, t4205, t4206, t4207, t4209, t4210, t4212, t4216)
}
