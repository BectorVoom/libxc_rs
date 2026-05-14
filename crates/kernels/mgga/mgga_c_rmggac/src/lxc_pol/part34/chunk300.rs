//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 300/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk300<F: Float>(t262: F, t3068: F, t2500: F, t2060: F, t664: F, t305: F, t128: F, t838: F, t28: F, t3046: F) -> (F, F, F, F, F, F) {
    let t3069 = t3068 * t262;
    let t3070 = t2500 * t3069;
    let t3072 = t2060 * t664;
    let t3074 = 0.2993560425465952141e-1 * t305 * t3072;
    let t3075 = t838 * t128;
    let t3076 = t28 * t3046;
    (t3069, t3070, t3072, t3074, t3075, t3076)
}
