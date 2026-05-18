//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 827/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk827<F: Float>(t15254: F, t16043: F, t2123: F, t3351: F, t515: F, t618: F, t7231: F, t1528: F, t664: F, t15258: F, t3352: F, t41059: F) -> (F, F, F, F, F) {
    let t74891 = t16043 * t15254;
    let t74896 = t3351 * t7231 * t515 * t2123 * t618;
    let t74901 = t3351 * t7231 * t515 * t664 * t1528;
    let t74903 = t16043 * t15258;
    let t74909 = t3351 * t3352 * t515 * t41059;
    (t74891, t74896, t74901, t74903, t74909)
}
