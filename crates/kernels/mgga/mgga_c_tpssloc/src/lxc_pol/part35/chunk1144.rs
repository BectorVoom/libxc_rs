//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1144/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1144<F: Float>(t22723: F, t268: F, t534: F, t22641: F, t3749: F, t1984: F, t80845: F, t2010: F, t6973: F, t80742: F, t154: F, t9533: F, t131: F, t3748: F, t2009: F, t9537: F) -> (F, F, F, F, F, F, F, F) {
    let t81046 = t22723 * t534 * t268;
    let t81064 = t22641 * t3749;
    let t81071 = t80845 * t1984;
    let t81072 = t81071 * t2010;
    let t81073 = 0.27720185200590482541e0 * t81072;
    let t81074 = t80742 * t6973;
    let t81075 = 0.16220877603642232915e0 * t81074;
    let t81142 = t9533 * t154;
    let t81144 = t81142 * t3748 * t131;
    let t81146 = t81144 * t9537 * t2009;
    (t81046, t81064, t81071, t81073, t81075, t81142, t81144, t81146)
}
