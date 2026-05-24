//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 771/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk771<F: Float>(t35844: F, t5259: F, t333: F, t7840: F, t4669: F, t128: F, t305: F, t3899: F, t265: F, t848: F, t262: F, t2073: F) -> (F, F, F, F, F, F, F) {
    let t35845 = t5259 * t35844;
    let t35847 = t7840 * t333;
    let t35848 = t4669 * t35847;
    let t35861 = t305 * t128 * t3899;
    let t35863 = t265 * t848;
    let t35864 = t262 * t35863;
    let t35865 = t2073 * t35864;
    (t35845, t35847, t35848, t35861, t35863, t35864, t35865)
}
