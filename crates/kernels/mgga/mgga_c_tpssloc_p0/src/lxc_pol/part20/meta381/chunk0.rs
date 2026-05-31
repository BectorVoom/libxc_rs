//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1738/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1738<F: Float>(t13191: F, t2701: F, t820: F, t1484: F, t2553: F, t2563: F, t4159: F, t119: F, t12971: F, t210: F, t4155: F, t9573: F) -> (F, F, F, F, F, F, F) {
    let t13193 = t2701 * t820 * t13191;
    let t13196 = t1484 * t2553;
    let t13198 = t2701 * t820 * t13196;
    let t13202 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t2563 * t4159;
    let t13203 = t119 * t12971;
    let t13204 = t210 * t13203;
    let t13208 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t9573 * t4155;
    (t13193, t13196, t13198, t13202, t13203, t13204, t13208)
}
