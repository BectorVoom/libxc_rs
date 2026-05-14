//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1182/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1182<F: Float>(t120172: F, t24996: F, t15868: F, t1983: F, t8489: F, t31246: F, t7754: F, t22751: F, t32731: F, t22633: F, t22635: F, t31099: F, t5187: F, t31090: F, t97721: F, t1377: F, t7749: F) -> (F, F, F, F, F, F, F) {
    let t120173 = t120172 * t24996;
    let t120176 = t1983 * t8489 * t15868;
    let t120177 = t31246 * t7754;
    let t120179 = t22751 * t32731;
    let t120180 = 0.76763589786250567037e-1 * t120179;
    let t120184 = 0.3289868133696452873e-1 * t22633 * t22635 * t31099 * t5187;
    let t120196 = 0.6579736267392905746e-1 * t22633 * t22635 * t31090 * t97721;
    let t120197 = t1377 * t7749;
    (t120173, t120176, t120177, t120180, t120184, t120196, t120197)
}
