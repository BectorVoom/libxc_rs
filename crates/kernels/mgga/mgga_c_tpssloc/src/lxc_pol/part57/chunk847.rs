//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 847/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk847<F: Float>(t118858: F, t1880: F, t214: F, t225: F, t258: F, t28406: F, t118910: F, t6552: F, t7479: F, t28276: F, t30663: F, t1484: F, t1527: F, t22986: F, t23270: F, t30633: F) -> (F, F, F, F, F, F) {
    let t126399 = 0.76763589786250567036e-1 * t118858;
    let t126404 = 0.16449340668482264365e-1 * t1880 * t214 * t28406 * t225 * t258;
    let t126409 = 0.6579736267392905746e-1 * t6552 * t118910 * t7479;
    let t126412 = 0.3289868133696452873e-1 * t6552 * t30663 * t28276;
    let t126413 = t1484 * t1527;
    let t126417 = 0.13159472534785811492e0 * t22986 * t23270 * t30633 * t126413;
    (t126399, t126404, t126409, t126412, t126413, t126417)
}
