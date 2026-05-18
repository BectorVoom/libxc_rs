//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1171/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1171<F: Float>(t31137: F, t6907: F, t1985: F, t6891: F, t6888: F, t1323: F, t8470: F, t6920: F, t8462: F, t1307: F, t1998: F, t59: F) -> (F, F, F, F, F, F, F) {
    let t31138 = t31137 * t6907;
    let t31140 = F::new(0.16449340668482264365e-1) * t1985 * t31138;
    let t31145 = t31137 * t6891;
    let t31147 = F::new(0.3289868133696452873e-1) * t6888 * t31145;
    let t31151 = t1323 * t8470;
    let t31153 = t6920 * t8462;
    let t31154 = F::new(0.56521858531796547196e-2) * t31153;
    let t31156 = t1998 * t59 * t1307;
    (t31138, t31140, t31145, t31147, t31151, t31154, t31156)
}
