//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 305/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk305<F: Float>(t1294: F, t763: F, t532: F, t571: F, t514: F, t517: F, t215: F, t535: F, t782: F, t154: F, t547: F) -> (F, F, F, F, F, F) {
    let t1296 = F::new(0.5848223622634646207e0) * t1294 * t763;
    let t1297 = t532 * t571;
    let t1298 = F::new(1.0) / t514;
    let t1302 = F::new(1.0) / t517;
    let t1313 = F::new(0.19444444444444444444e-2) * t782 * t535 * t215;
    let t1314 = t154 * t547;
    (t1296, t1297, t1298, t1302, t1313, t1314)
}
