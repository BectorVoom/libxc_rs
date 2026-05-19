//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 864/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk864<F: Float>(t125: F, t755: F, t123: F, t128: F, t121: F, t22: F, t2196: F, t7823: F, t667: F, t7826: F, t124: F, t138: F, t599: F) -> (F, F, F, F, F, F) {
    let t7829 = t125 * t755;
    let t7830 = t123 * t7829;
    let t7832 = F::new(1.0)/pow_3_2::<F>(t128);
    let t7833 = t7832 * t121;
    let t7834 = t7833 * t22;
    let t7836 = t2196 * t7823;
    let t7838 = t667 * t7826;
    let t7841 = t138 * t124 * t599;
    (t7829, t7830, t7834, t7836, t7838, t7841)
}
