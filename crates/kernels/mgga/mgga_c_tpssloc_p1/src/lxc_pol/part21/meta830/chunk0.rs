//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2926/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2926<F: Float>(t17297: F, t2904: F, t952: F, t959: F, t300: F, t59774: F, t17304: F, t2940: F, t2929: F, t2932: F, t59975: F, t60037: F, t60039: F, t60041: F, t60044: F, t60047: F, t60050: F, t60053: F, t60056: F, t60354: F) -> (F, F, F, F, F) {
    let t60915 = F::cast_from(0.23392894490538584828e1_f64) * t959 * t2904 * t17297 * t952;
    let t60917 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t59774;
    let t60919 = F::cast_from(0.70178683471615754484e1_f64) * t2940 * t17304;
    let t60923 = F::cast_from(0.34631718211362927518e2_f64) * t959 * t2929 * t59975 * t2932;
    let t60924 = -t60037 + t60039 + t60041 + t60044 + t60047 - t60050 - t60053 - t60056 + t60915 + t60917 + t60354 - t60919 - t60923;
    (t60915, t60917, t60919, t60923, t60924)
}
