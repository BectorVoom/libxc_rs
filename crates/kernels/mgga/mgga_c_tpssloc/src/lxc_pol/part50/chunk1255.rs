//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1255/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1255<F: Float>(t1992: F, t22635: F, t31090: F, t5353: F, t114160: F, t6888: F, t7691: F, t26189: F, t31137: F, t31169: F, t5234: F, t31172: F) -> (F, F, F, F) {
    let t120334 = F::new(0.3289868133696452873e-1) * t1992 * t22635 * t31090 * t5353;
    let t120337 = F::new(0.3289868133696452873e-1) * t6888 * t114160 * t7691;
    let t120340 = F::new(0.3289868133696452873e-1) * t6888 * t31137 * t26189;
    let t120341 = t5234 * t31169;
    let t120342 = t120341 * t31172;
    (t120334, t120337, t120340, t120342)
}
