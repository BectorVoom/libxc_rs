//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 387/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk387<F: Float>(t3127: F, t363: F, t3037: F, t3033: F, t360: F, t2770: F, t2978: F, t2775: F, t976: F, t221: F, t2965: F, t339: F) -> (F, F, F, F, F, F) {
    let t3128 = t3127 * t363;
    let t3129 = t3128 * t3037;
    let t3130 = t3033 * t3129;
    let t3131 = t360 * t360;
    let t3146 = t2978 * t2770;
    let t3151 = t976 * t2775;
    let t3158 = t221 * t2965;
    let t3160 = t339 * t3158 / F::cast_from(432.0_f64);
    (t3130, t3131, t3146, t3151, t3158, t3160)
}
