//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 829/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk829<F: Float>(t15252: F, t495: F, t515: F, t7230: F, t7231: F, t3154: F, t9090: F, t13832: F, t61965: F, t13836: F, t38495: F, t321: F, t3351: F, t7248: F) -> (F, F, F, F, F) {
    let t74927 = F::cast_from(0.1064114997332445985e-4_f64) * t7230 * t7231 * t515 * t15252 * t495;
    let t74928 = t9090 * t3154;
    let t74929 = F::cast_from(0.19863479950205658386e-4_f64) * t74928;
    let t74930 = t61965 * t13832;
    let t74932 = t38495 * t13836;
    let t74943 = t3351 * t7248 * t515 * t15252 * t321;
    (t74927, t74929, t74930, t74932, t74943)
}
