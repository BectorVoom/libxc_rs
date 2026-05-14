//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 424/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk424<F: Float>(t236: F, t3787: F, t550: F, t1339: F, t835: F, t1336: F, t1354: F, t242: F, t1365: F, t67: F, t246: F, t120: F, t1351: F, t1307: F, t1291: F, t2663: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3788 = t3787 * t236;
    let t3792 = t550 * t550;
    let t3798 = t1339 * t835;
    let t3799 = t1336 * t3798;
    let t3800 = t3799 * t1354;
    let t3802 = t1339 * t242;
    let t3803 = t1336 * t3802;
    let t3804 = t1365 * t67;
    let t3805 = t3804 * t246;
    let t3806 = t120 * t1351;
    let t3807 = t550 * t1307;
    let t3813 = 0.24415263074675393405e-3 * t1291 * t2663;
    (t3788, t3792, t3799, t3800, t3803, t3805, t3806, t3807, t3813)
}
