//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1013/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1013<F: Float>(t10966: F, t2534: F, t1425: F, t8590: F, t2476: F, t3807: F, t1408: F, t2193: F) -> (F, F, F, F) {
    let t10968 = F::cast_from(0.16081979498692535067e2_f64) * t10966 * t2534;
    let t10970 = F::new(1.0) * t8590 * t1425;
    let t10972 = F::new(2.0) * t2476 * t3807;
    let t10980 = t2193 * t1408;
    (t10968, t10970, t10972, t10980)
}
