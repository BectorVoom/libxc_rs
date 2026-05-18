//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 379/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk379<F: Float>(t2084: F, t321: F, t27: F, t1343: F, t265: F, t71: F, t1330: F, t271: F, t270: F, t303: F, t357: F, t36: F, t4789: F) -> (F, F, F, F, F, F, F) {
    let t7287 = t2084 * t321;
    let t7288 = t27 * t7287;
    let t7292 = t265 * t1343 * t71;
    let t7296 = t1330 * t271;
    let t7297 = t7296 * t71;
    let t7301 = t303 * t270;
    let t7305 = t357 * t270;
    let t7310 = t36 * t4789 * t71;
    (t7288, t7292, t7296, t7297, t7301, t7305, t7310)
}
