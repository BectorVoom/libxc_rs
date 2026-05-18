//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 675/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk675<F: Float>(t14122: F, t68503: F, t118: F, t13861: F, t201: F, t209: F, t476: F, t13980: F, t2160: F, t638: F, t13984: F, t3061: F, t7184: F) -> (F, F, F, F, F, F) {
    let t68504 = t14122 * t68503;
    let t68505 = t13861 * t118;
    let t68508 = t201 * t476 * t209;
    let t68514 = t638 * t2160 * t13980;
    let t68517 = t638 * t2160 * t13984;
    let t68520 = t638 * t7184 * t3061;
    (t68504, t68505, t68508, t68514, t68517, t68520)
}
