//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 531/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk531<F: Float>(t1043: F, t154: F, t632: F, t2289: F, t888: F, t892: F, t287: F, t891: F, t275: F, t273: F, t276: F, t2764: F) -> (F, F, F, F, F, F, F) {
    let t2768 = t154 * t1043;
    let t2769 = t632 * t632;
    let t2770 = F::cast_from(1.0_f64) / t2769;
    let t2775 = F::cast_from(1.0_f64) / t2289;
    let t2787 = t888 * t892;
    let t2790 = t891 * t287;
    let t2791 = F::cast_from(1.0_f64) / t2790;
    let t2792 = t275 * t2791;
    let t2798 = F::cast_from(1.0_f64) / t276 / t273;
    let t2802 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2764;
    (t2768, t2770, t2775, t2787, t2792, t2798, t2802)
}
