//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 634/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk634<F: Float>(t2040: F, t2046: F, t36938: F, t14091: F, t35244: F, t35228: F, t3154: F, t34881: F, t14051: F, t14367: F, t14053: F, t2145: F, t27: F, t3118: F, t664: F, t14140: F, t7297: F) -> (F, F, F, F, F, F, F, F) {
    let t70021 = t2046 * t36938 * t2040;
    let t70048 = t14091 * t35244;
    let t70050 = t14091 * t35228;
    let t70052 = t34881 * t3154;
    let t70062 = t14051 * t14367;
    let t70063 = t70062 * t14053;
    let t70071 = t2145 * t27 * t3118 * t664;
    let t70078 = t2046 * t7297 * t14140;
    (t70021, t70048, t70050, t70052, t70062, t70063, t70071, t70078)
}
