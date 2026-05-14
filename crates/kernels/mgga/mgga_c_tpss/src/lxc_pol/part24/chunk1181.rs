//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1181/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1181<F: Float>(t19724: F, t219: F, t6131: F, t1705: F, t3692: F, t935: F, t5570: F, t6134: F) -> (F, F, F, F, F) {
    let t19725 = param_beta * t19724;
    let t19727 = t6131 * t219;
    let t19733 = t1705 * t3692;
    let t19734 = t19733 * t935;
    let t19736 = t6134 * t5570;
    (t19725, t19727, t19733, t19734, t19736)
}
