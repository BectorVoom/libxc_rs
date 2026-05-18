//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 733/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk733<F: Float>(t14639: F, t2186: F, t14563: F, t2019: F, t2020: F, t270: F, t702: F, t31: F, t7349: F, t7351: F, t14683: F, t7244: F) -> (F, F, F, F, F) {
    let t70885 = t2186 * t14639;
    let t70892 = t2019 * t2020 * t14563;
    let t70901 = t702 * t270;
    let t70904 = t7349 * t7351 * t70901 * t31;
    let t70905 = F::new(0.43368970657079495312e-4) * t70904;
    let t70929 = t7244 * t14683;
    (t70885, t70892, t70901, t70905, t70929)
}
