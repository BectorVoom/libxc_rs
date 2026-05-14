//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 322/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk322<F: Float>(t22: F, t698: F, t656: F, t3203: F, t515: F, t1968: F, t1978: F) -> (F, F, F, F) {
    let t3224 = t698 * t22;
    let t3225 = t3224 * t656;
    let t3230 = t515 * t3203;
    let t3350 = t1978 * t1968;
    (t3224, t3225, t3230, t3350)
}
