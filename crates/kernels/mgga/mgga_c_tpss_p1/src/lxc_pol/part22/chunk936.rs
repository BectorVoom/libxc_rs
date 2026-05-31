//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 936/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk936<F: Float>(t3000: F, t433: F, t275: F, t400: F, t8662: F, t2896: F, t673: F, t235: F, t3032: F, t2839: F, t610: F, t1039: F, t2202: F) -> (F, F, F, F, F, F, F) {
    let t9176 = F::cast_from(1.0_f64) / t3000 / t433;
    let t9181 = t275 * t8662 * t400;
    let t9182 = F::cast_from(0.36793333333333333333e0_f64) * t9181;
    let t9183 = t673 * t2896;
    let t9185 = t235 * t3032;
    let t9187 = F::cast_from(1.0_f64) / t2839 / t610;
    let t9192 = t2202 * t1039;
    (t9176, t9181, t9182, t9183, t9185, t9187, t9192)
}
