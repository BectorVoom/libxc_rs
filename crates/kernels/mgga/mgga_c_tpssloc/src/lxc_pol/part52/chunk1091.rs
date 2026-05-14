//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1091/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1091<F: Float>(t32734: F, t32780: F, t533: F, t1390: F, t1983: F, t30663: F, t7479: F, t6552: F, t7488: F, t1880: F, t1527: F, t8352: F, t10110: F, t1911: F, t7537: F, t2718: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32781 = t32734 + t32780;
    let t32782 = t533 * t32781;
    let t32783 = t32782 * t1390;
    let t32784 = t1983 * t32783;
    let t32789 = t30663 * t7479;
    let t32791 = 0.3289868133696452873e-1 * t6552 * t32789;
    let t32792 = t30663 * t7488;
    let t32794 = 0.16449340668482264365e-1 * t1880 * t32792;
    let t32795 = t8352 * t1527;
    let t32796 = t10110 * t32795;
    let t32799 = t1911 * t7537;
    let t32800 = t2718 * t32799;
    (t32781, t32782, t32783, t32784, t32789, t32791, t32792, t32794, t32796, t32800)
}
