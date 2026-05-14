//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 728/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk728<F: Float>(t69619: F, t75148: F, t14225: F, t3352: F, t8446: F, t15397: F, t495: F, t2067: F, t3369: F, t70460: F, t15280: F, t325: F, t14170: F, t14131: F, t21714: F, t9152: F) -> (F, F, F, F, F) {
    let t75149 = t69619 * t75148;
    let t75152 = t14225 * t3352 * t8446;
    let t75154 = t15397 * t495;
    let t75157 = t70460 * t3369 * t2067 * t75154;
    let t75162 = t15280 * t325;
    let t75163 = t75162 * t14170;
    let t75166 = t14131 * t21714 * t9152;
    (t75149, t75152, t75157, t75163, t75166)
}
