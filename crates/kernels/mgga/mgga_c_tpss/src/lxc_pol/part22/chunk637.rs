//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 637/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk637<F: Float>(t2834: F, t392: F, t2869: F, t1032: F, t2877: F, t2509: F, t275: F, t400: F, t1039: F, t673: F) -> (F, F, F, F, F, F, F) {
    let t2880 = F::new(0.39862222222222222223e0) * t2834;
    let t2885 = F::new(1.0)/f64::sqrt(t392);
    let t2886 = t2885 * t2869;
    let t2888 = t1032 * t2877;
    let t2891 = t275 * t2509 * t400;
    let t2892 = F::new(0.13692777777777777778e0) * t2891;
    let t2893 = t673 * t1039;
    (t2880, t2885, t2886, t2888, t2891, t2892, t2893)
}
