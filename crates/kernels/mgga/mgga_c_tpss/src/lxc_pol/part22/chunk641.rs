//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 641/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk641<F: Float>(t407: F, t2863: F, t2911: F, t2834: F, t2836: F, t2843: F, t2848: F, t2852: F, t1049: F, t1053: F, t1052: F, t417: F) -> (F, F, F, F, F, F, F, F) {
    let t2912 = t407 * t407;
    let t2913 = F::new(1.0) / t2912;
    let t2914 = t2863 * t2913;
    let t2916 = F::cast_from(0.16081979498692535067e2_f64) * t2911 * t2914;
    let t2917 = F::cast_from(0.22831111111111111111e-1_f64) * t2834;
    let t2922 = t2917 - F::cast_from(0.11415555555555555555e-1_f64) * t2836 - F::cast_from(0.11415555555555555555e-1_f64) * t2843 + F::cast_from(0.34246666666666666666e-1_f64) * t2848 + F::cast_from(0.17123333333333333333e-1_f64) * t2852;
    let t2925 = t1049 * t1053;
    let t2928 = t1052 * t417;
    (t2912, t2913, t2914, t2916, t2917, t2922, t2925, t2928)
}
