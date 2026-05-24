//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 875/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk875<F: Float>(t562: F, t65: F, t197: F, t509: F, t1270: F, t1844: F, t64: F, t789: F, t112: F, t2023: F, t641: F, t2075: F, t600: F) -> (F, F, F, F, F, F, F) {
    let t7091 = F::new(1.0) / t65 / t562;
    let t7309 = t197 * t509;
    let t7383 = t1844 * t1270;
    let t7585 = t64 * t789;
    let t7587 = F::new(154.0) / F::new(27.0) * t7585 * t112;
    let t7588 = t2023 * t641;
    let t7590 = t600 * t2075;
    (t7091, t7309, t7383, t7585, t7587, t7588, t7590)
}
