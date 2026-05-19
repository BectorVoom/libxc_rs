//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 933/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk933<F: Float>(t22674: F, t31123: F, t6897: F, t1992: F, t22635: F, t31090: F, t3911: F, t214: F, t6955: F, t1985: F, t6907: F, t80707: F, t8458: F) -> (F, F, F, F, F) {
    let t114154 = t6897 * t22674 * t31123;
    let t114155 = F::cast_from(0.16449340668482264365e-1_f64) * t114154;
    let t114159 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t22635 * t31090 * t3911;
    let t114160 = t214 * t6955;
    let t114163 = F::cast_from(0.3289868133696452873e-1_f64) * t1985 * t114160 * t6907;
    let t114168 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t80707 * t8458;
    (t114155, t114159, t114160, t114163, t114168)
}
