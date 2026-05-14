//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1004/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1004<F: Float>(t3762: F, t845: F, t867: F, t2526: F, t3765: F, t1411: F, t2530: F, t2534: F, t1425: F, t8590: F, t2476: F, t3807: F, t1408: F, t2193: F) -> (F, F, F, F, F, F) {
    let t10961 = t3762 * t845;
    let t10963 = 2.0 * t10961 * t867;
    let t10965 = 1.0 * t3765 * t2526;
    let t10966 = t1411 * t2530;
    let t10968 = 0.16081979498692535067e2 * t10966 * t2534;
    let t10970 = 1.0 * t8590 * t1425;
    let t10972 = 2.0 * t2476 * t3807;
    let t10980 = t2193 * t1408;
    (t10963, t10965, t10968, t10970, t10972, t10980)
}
