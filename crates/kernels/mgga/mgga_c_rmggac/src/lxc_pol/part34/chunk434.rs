//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 434/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk434<F: Float>(t262: F, t8901: F, t7788: F, t2392: F, t333: F, t7782: F, t7835: F, t8622: F, t2068: F, t8709: F, t2073: F, t8713: F) -> (F, F, F, F, F, F, F, F) {
    let t8902 = t262 * t8901;
    let t8903 = t7788 * t8902;
    let t8905 = t2392 * t333;
    let t8906 = t262 * t8905;
    let t8907 = t7782 * t8906;
    let t8909 = t7835 * t8622;
    let t8911 = t2068 * t8709;
    let t8913 = t2073 * t8713;
    (t8902, t8903, t8905, t8906, t8907, t8909, t8911, t8913)
}
