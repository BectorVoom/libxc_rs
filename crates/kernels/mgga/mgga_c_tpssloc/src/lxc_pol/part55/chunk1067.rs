//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1067/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1067<F: Float>(t1484: F, t865: F, t22986: F, t23270: F, t30633: F, t112867: F, t1880: F, t23237: F, t32866: F, t1888: F, t4300: F, t25216: F, t30663: F, t112892: F, t32792: F, t6547: F) -> (F, F, F, F, F, F, F) {
    let t118833 = t1484 * t865;
    let t118837 = 0.6579736267392905746e-1 * t22986 * t23270 * t30633 * t118833;
    let t118838 = 0.16449340668482264365e-1 * t112867;
    let t118841 = 0.16449340668482264365e-1 * t1880 * t23237 * t32866;
    let t118847 = 0.3289868133696452873e-1 * t1888 * t23270 * t30633 * t4300;
    let t118850 = 0.16449340668482264365e-1 * t1880 * t30663 * t25216;
    let t118851 = 0.82246703342411321825e-2 * t112892;
    let t118858 = t6547 * t32792;
    (t118837, t118838, t118841, t118847, t118850, t118851, t118858)
}
