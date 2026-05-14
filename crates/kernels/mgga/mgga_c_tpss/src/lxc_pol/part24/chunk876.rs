//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 876/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk876<F: Float>(t720: F, t7813: F, t121: F, t131: F, t141: F, t22: F, t2185: F, t599: F, t2184: F, t660: F, t755: F, t659: F, t125: F, t123: F, t128: F, t2196: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7814 = t7813 * t720;
    let t7820 = 1.0 / t131 / t141 * t121 / 4.0;
    let t7821 = t7820 * t22;
    let t7823 = t2185 * t599;
    let t7824 = t2184 * t7823;
    let t7826 = t660 * t755;
    let t7827 = t659 * t7826;
    let t7829 = t125 * t755;
    let t7830 = t123 * t7829;
    let t7832 = 1.0/pow_3_2(t128);
    let t7833 = t7832 * t121;
    let t7834 = t7833 * t22;
    let t7836 = t2196 * t7823;
    (t7814, t7821, t7824, t7826, t7827, t7829, t7830, t7834, t7836)
}
