//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1130/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1130<F: Float>(t32761: F, t6897: F, t794: F, t114208: F, t114216: F, t114285: F, t1992: F, t26355: F, t114240: F, t114242: F, t114172: F, t7700: F, t22674: F, t32697: F, t114253: F, t114225: F, t1375: F, t1842: F, t2015: F, t22656: F, t26224: F, t26225: F, t26347: F, t26471: F, t31189: F, t31216: F, t3887: F, t5210: F, t5354: F, t568: F, t7729: F, t8470: F) -> (F,) {
    let t120550 = t6897 * t794 * t32761;
    let t120551 = 0.82246703342411321825e-2 * t120550;
    let t120552 = 0.76763589786250567036e-1 * t114208;
    let t120553 = 0.76763589786250567036e-1 * t114216;
    let t120556 = 0.3289868133696452873e-1 * t1992 * t114285 * t26355;
    let t120561 = 0.16449340668482264365e-1 * t114240;
    let t120566 = 0.38381794893125283518e-1 * t114242;
    let t120568 = t6897 * t114172 * t7700;
    let t120569 = 0.82246703342411321825e-2 * t120568;
    let t120576 = t6897 * t22674 * t32697;
    let t120577 = 0.82246703342411321825e-2 * t120576;
    let t120579 = 0.38381794893125283518e-1 * t114253;
    let t120582 = 2.0 * t1375 * t1842 * t31216 * t3887 + 4.0 * t1375 * t2015 * t26471 * t3887 - 12.0 * t26224 * t26225 * t26347 + t5210 * t568 * t8470 + 4.0 * t22656 * t7729 - t31189 * t5354 + t114225 - t120551 - t120552 + t120553 + t120556 - t120561 - t120566 + t120569 + t120577 + t120579;
    (t120582,)
}
