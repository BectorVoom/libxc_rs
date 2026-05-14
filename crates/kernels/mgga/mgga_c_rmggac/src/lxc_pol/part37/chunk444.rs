//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 444/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk444<F: Float>(t3046: F, t321: F, t1326: F, t13911: F, t1322: F, t3839: F) -> (F, F, F, F) {
    let t13912 = t3046 * t321;
    let t13913 = t1326 * t13912;
    let t13914 = t13911 * t13913;
    let t13916 = t3839 * t1322;
    (t13912, t13913, t13914, t13916)
}
