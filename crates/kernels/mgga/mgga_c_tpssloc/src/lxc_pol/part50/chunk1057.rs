//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1057/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1057<F: Float>(t112660: F, t6552: F, t7479: F, t112961: F, t32823: F, t6579: F, t1888: F, t22996: F, t25281: F, t1509: F, t8347: F, t1484: F, t1902: F, t22986: F, t6646: F, t829: F) -> (F, F, F, F, F, F, F) {
    let t118672 = 0.3289868133696452873e-1 * t6552 * t112660 * t7479;
    let t118677 = 0.16449340668482264365e-1 * t112961;
    let t118678 = t6579 * t32823;
    let t118679 = 0.38381794893125283518e-1 * t118678;
    let t118682 = 0.3289868133696452873e-1 * t1888 * t22996 * t25281;
    let t118684 = t8347 * t1509;
    let t118690 = t1902 * t1484;
    let t118694 = 0.3289868133696452873e-1 * t22986 * t6646 * t118690 * t829;
    (t118672, t118677, t118679, t118682, t118684, t118690, t118694)
}
