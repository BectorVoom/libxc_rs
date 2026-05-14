//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 762/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk762<F: Float>(t345: F, t8400: F, t383: F, t8391: F, t1920: F, t353: F, t1055: F) -> (F, F, F, F) {
    let t8401 = t345 * t8400;
    let t8404 = t383 * t8391;
    let t8406 = 0.16449340668482264365e-1 * t1920 * t8401 + t353 * t8404;
    let t8407 = t1055 * t8406;
    (t8401, t8404, t8406, t8407)
}
