//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 982/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk982<F: Float>(t1984: F, t80845: F, t2010: F, t6973: F, t80742: F, t154: F, t9533: F, t131: F, t3748: F, t2009: F, t9537: F, t2690: F, t22691: F, t1887: F, t22797: F, t22715: F, t6887: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t81071 = t80845 * t1984;
    let t81072 = t81071 * t2010;
    let t81074 = t80742 * t6973;
    let t81142 = t9533 * t154;
    let t81144 = t81142 * t3748 * t131;
    let t81146 = t81144 * t9537 * t2009;
    let t81151 = t2690 * t154;
    let t81152 = t81151 * t3748;
    let t81153 = t81152 * t22691;
    let t81159 = t22797 * t1887;
    let t81186 = t22715 * t6887;
    (t81071, t81072, t81074, t81142, t81144, t81146, t81151, t81152, t81153, t81159, t81186)
}
