//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1149/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1149<F: Float>(t40995: F, t41037: F, t41077: F, t41120: F, t41343: F, t41393: F, t41434: F, t41487: F, t10069: F, t10077: F, t10080: F, t10091: F, t10098: F, t13390: F, t13397: F, t226: F, t22997: F, t235: F, t2617: F, t2728: F, t2732: F, t40926: F, t40932: F, t40934: F, t40938: F, t40951: F, t40955: F, t4291: F, t812: F, t829: F, t9958: F) -> (F, F) {
    let t41490 = t40995 + t41037 + t41077 + t41120 + t41343 + t41393 + t41434 + t41487;
    let t41495 = -36.0 * t10080 * t40938 * t812 - 36.0 * t13397 * t22997 * t40951 + t226 * t235 * t41490 + 14.0 * t2728 * t40926 * t812 - 4.0 * t2732 * t812 * t9958 + 24.0 * t40932 * t40934 * t812 - 12.0 * t40955 * t4291 * t829 - 4.0 * t10069 * t2617 - 12.0 * t10077 * t2617 - 12.0 * t10091 * t2617 - 12.0 * t10098 * t13390;
    (t41490, t41495)
}
