//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 948/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk948<F: Float>(t1971: F, t3351: F, t46005: F, t875: F, t7720: F, t9731: F, t674: F, t7715: F, t9734: F, t1997: F, t2004: F, t45561: F, t39277: F, t9118: F, t1910: F, t352: F, t515: F, t7231: F) -> (F, F, F, F, F, F) {
    let t47866 = t3351 * t1971 * t875 * t46005;
    let t47868 = t7720 * t9731;
    let t47871 = t9734 * t7715 * t674;
    let t47872 = t47871 * t1997;
    let t47874 = t45561 * t2004;
    let t47876 = t39277 * t9118;
    let t47881 = t3351 * t7231 * t515 * t1910 * t352;
    (t47866, t47868, t47872, t47874, t47876, t47881)
}
