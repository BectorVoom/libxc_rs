//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1247/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1247<F: Float>(t120179: F, t22633: F, t22635: F, t31099: F, t5187: F, t31090: F, t97721: F, t1377: F, t7749: F, t1307: F, t225: F, t32708: F) -> (F, F, F, F, F) {
    let t120180 = F::new(0.76763589786250567037e-1) * t120179;
    let t120184 = F::new(0.3289868133696452873e-1) * t22633 * t22635 * t31099 * t5187;
    let t120196 = F::new(0.6579736267392905746e-1) * t22633 * t22635 * t31090 * t97721;
    let t120197 = t1377 * t7749;
    let t120201 = F::new(0.3289868133696452873e-1) * t22633 * t22635 * t120197 * t1307;
    let t120203 = t32708 * t225;
    (t120180, t120184, t120196, t120201, t120203)
}
