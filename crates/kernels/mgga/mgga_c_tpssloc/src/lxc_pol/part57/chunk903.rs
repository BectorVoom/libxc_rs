//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 903/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk903<F: Float>(t22633: F, t22635: F, t31549: F, t6347: F, t22685: F, t28191: F, t31611: F, t1985: F, t8621: F, t97511: F, t102922: F, t122152: F, t127203: F, t127210: F, t1375: F, t1842: F, t2016: F, t2091: F, t2092: F, t26224: F, t27009: F, t28186: F, t28220: F, t28223: F, t28224: F, t29361: F, t33293: F, t33301: F, t33316: F, t3887: F, t5321: F, t6958: F, t7194: F, t7729: F, t93319: F, t97756: F) -> (F,) {
    let t128671 = t22633 * t22635 * t31549 * t6347;
    let t128691 = t22685 * t31611 * t28191;
    let t128694 = t1985 * t97511 * t8621;
    let t128701 = 4.0 * t5321 * t33316 + 4.0 * t7194 * t28220 - t127203 + 0.16449340668482264365e-1 * t128671 - 0.38381794893125283518e-1 * t122152 + 4.0 * t1375 * t3887 * t33293 * t1842 + t127210 + 4.0 * t27009 * t7729 + 2.0 * t1375 * t3887 * t2091 * t28186 + 4.0 * t5321 * t33301 - 2.0 * t97756 * t2092 - 6.0 * t7194 * t28224 + 0.49348022005446793095e-1 * t128691 - 0.82246703342411321825e-2 * t128694 - t6958 * t29361 - t102922 * t2016 + 24.0 * t26224 * t93319 * t28223;
    (t128701,)
}
