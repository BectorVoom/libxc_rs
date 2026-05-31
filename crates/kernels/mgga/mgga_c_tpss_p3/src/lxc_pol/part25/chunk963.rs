//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 963/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk963<F: Float>(t10117: F, t4425: F, t4466: F, t3342: F, t4484: F, t1646: F, t9994: F, t10137: F, t4405: F, t219: F, t4488: F, t10085: F, t220: F, t73: F) -> (F, F, F, F, F, F, F) {
    let t13004 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t10117 * t4425;
    let t13006 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t10117 * t4466;
    let t13013 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t3342 * t4484;
    let t13018 = t9994 * t1646;
    let t13021 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t10137 * t4405;
    let t13035 = t4488 * t219;
    let t13059 = t220 * t73 * t10085;
    (t13004, t13006, t13013, t13018, t13021, t13035, t13059)
}
