//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2246/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2246<F: Float>(t5889: F, t698: F, t973: F, t10422: F, t17676: F, t3070: F, t17171: F, t2970: F, t17167: F, t10231: F, t17157: F, t17161: F) -> (F, F, F, F, F, F) {
    let t62565 = t973 * t698 * t5889;
    let t62602 = t3070 * t10422 * t17676;
    let t62631 = t973 * t2970 * t17171;
    let t62640 = t973 * t2970 * t17167;
    let t62657 = t973 * t10231 * t17157;
    let t62660 = t973 * t10231 * t17161;
    (t62565, t62602, t62631, t62640, t62657, t62660)
}
