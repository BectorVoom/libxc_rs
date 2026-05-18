//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1046/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1046<F: Float>(t28163: F, t6976: F, t1992: F, t19660: F, t550: F, t19743: F, t6330: F, t6890: F, t6889: F, t22685: F, t26193: F, t7700: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28164 = t6976 * t28163;
    let t28165 = t1992 * t28164;
    let t28167 = t19660 * t550;
    let t28168 = t6976 * t28167;
    let t28169 = t1992 * t28168;
    let t28181 = t19743 * t550;
    let t28182 = t6976 * t28181;
    let t28183 = t1992 * t28182;
    let t28191 = t6890 * t6330;
    let t28192 = t6889 * t28191;
    let t28193 = t22685 * t28192;
    let t28195 = t26193 * t7700;
    (t28164, t28165, t28167, t28168, t28169, t28181, t28182, t28183, t28191, t28192, t28193, t28195)
}
