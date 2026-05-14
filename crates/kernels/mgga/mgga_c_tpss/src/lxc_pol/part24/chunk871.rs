//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 871/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk871<F: Float>(t3: F, t6279: F, t1668: F, t1786: F, t1338: F, t5772: F, t547: F, t117: F, t6112: F, t1670: F, t1784: F, t548: F, t1777: F, t3205: F) -> (F, F, F, F, F, F) {
    let t6280 = t3 * t6279;
    let t6284 = param_d * t6279;
    let t6289 = 3.0 * t1668 * t1786;
    let t6290 = t5772 * t1338;
    let t6292 = 6.0 * t547 * t6290;
    let t6293 = t117 * t6112;
    let t6295 = 3.0 * t547 * t6293;
    let t6296 = 3.0 * t1670 * t1784 + t548 * t6284 + t6289 + t6292 + t6295;
    let t7029 = t3205 * t1777;
    (t6280, t6284, t6290, t6293, t6296, t7029)
}
