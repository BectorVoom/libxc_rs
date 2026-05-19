//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 878/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk878<F: Float>(t14022: F, t14027: F, t1451: F, t201: F, t3112: F, t13862: F, t14032: F, t75027: F, t15075: F, t30080: F, t15382: F, t498: F, t515: F, t7230: F, t7231: F) -> (F, F, F, F) {
    let t75700 = t3112 * t1451 * t201 * t14022 * t14027;
    let t75703 = t14032 * t13862 * t75027;
    let t75705 = t30080 * t15075;
    let t75718 = F::cast_from(0.1064114997332445985e-4_f64) * t7230 * t7231 * t515 * t15382 * t498;
    (t75700, t75703, t75705, t75718)
}
