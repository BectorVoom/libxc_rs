//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 870/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk870<F: Float>(t509: F, t6273: F, t1270: F, t1760: F, t1778: F, t4525: F, t1668: F, t1786: F, t1338: F, t5772: F, t547: F, t117: F, t6112: F, t1289: F, t1299: F, t5500: F, t5971: F, t61: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6274 = t509 * t6273;
    let t6275 = t6274 * t1270;
    let t6276 = t1760 * t6275;
    let t6277 = t1778 * t4525;
    let t6278 = t1760 * t6277;
    let t6289 = 3.0 * t1668 * t1786;
    let t6290 = t5772 * t1338;
    let t6292 = 6.0 * t547 * t6290;
    let t6293 = t117 * t6112;
    let t6295 = 3.0 * t547 * t6293;
    let t6470 = -8.0 / 3.0 * t1299 * t61 - 5.0 / 6.0 * t5971 * t1289 + t5500;
    (t6274, t6275, t6276, t6277, t6278, t6289, t6290, t6292, t6293, t6295, t6470)
}
