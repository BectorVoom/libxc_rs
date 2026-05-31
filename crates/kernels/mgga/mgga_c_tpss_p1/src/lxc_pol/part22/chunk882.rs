//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 882/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk882<F: Float>(t720: F, t7813: F, t121: F, t131: F, t141: F, t22: F, t2185: F, t599: F, t2184: F, t660: F, t755: F, t659: F) -> (F, F, F, F, F, F) {
    let t7814 = t7813 * t720;
    let t7820 = F::cast_from(1.0_f64) / t131 / t141 * t121 / F::cast_from(4.0_f64);
    let t7821 = t7820 * t22;
    let t7823 = t2185 * t599;
    let t7824 = t2184 * t7823;
    let t7826 = t660 * t755;
    let t7827 = t659 * t7826;
    (t7814, t7821, t7823, t7824, t7826, t7827)
}
