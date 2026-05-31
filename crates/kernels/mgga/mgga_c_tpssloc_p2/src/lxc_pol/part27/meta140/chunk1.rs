//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 797/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk797<F: Float>(t3082: F, t370: F, t35: F, t365: F, t612: F, t364: F, t354: F, t1032: F, t1036: F, t1004: F, t1031: F, t1044: F, t248: F, t2776: F) -> (F, F, F, F, F, F, F) {
    let t3084 = t370 * t3082 / F::cast_from(13824.0_f64);
    let t3087 = F::cast_from(1.0_f64) / t35 / t365 / t612;
    let t3088 = t364 * t3087;
    let t3089 = t354 * t3088;
    let t3092 = t1032 * t1036;
    let t3094 = t1004 * t1031;
    let t3098 = t248 * t1044 * t2776;
    (t3084, t3087, t3088, t3089, t3092, t3094, t3098)
}
