//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 244/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk244<F: Float>(t2012: F, t291: F, t20: F, t253: F, t22: F, t259: F, t26: F) -> (F, F, F, F, F) {
    let t2013 = t2012 * t291;
    let t2016 = t253 * t20;
    let t2017 = t259 * t22;
    let t2018 = t2017 * t26;
    let t2019 = t2016 * t2018;
    (t2013, t2016, t2017, t2018, t2019)
}
