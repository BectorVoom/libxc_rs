//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 989/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk989<F: Float>(t30663: F, t7479: F, t6552: F, t7488: F, t1880: F, t1527: F, t8352: F, t10110: F, t1911: F, t7537: F, t2718: F, t8362: F, t225: F, t258: F, t7510: F, t214: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32789 = t30663 * t7479;
    let t32791 = 0.3289868133696452873e-1 * t6552 * t32789;
    let t32792 = t30663 * t7488;
    let t32794 = 0.16449340668482264365e-1 * t1880 * t32792;
    let t32795 = t8352 * t1527;
    let t32796 = t10110 * t32795;
    let t32799 = t1911 * t7537;
    let t32800 = t2718 * t32799;
    let t32803 = t8362 * t1527;
    let t32804 = t2718 * t32803;
    let t32808 = t7510 * t225 * t258;
    let t32809 = t214 * t32808;
    (t32789, t32791, t32792, t32794, t32796, t32800, t32804, t32808, t32809)
}
