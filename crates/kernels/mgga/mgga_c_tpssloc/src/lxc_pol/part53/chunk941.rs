//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 941/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk941<F: Float>(t1351: F, t22705: F, t22852: F, t550: F, t59: F, t31338: F, t81651: F, t82074: F, t2717: F, t7106: F, t31334: F, t6579: F) -> (F, F, F, F) {
    let t114046 = t22852 * t22705 * t59 * t1351 * t550;
    let t114592 = t81651 * t82074 * t31338;
    let t114601 = t2717 * t7106;
    let t114606 = t6579 * t31334;
    (t114046, t114592, t114601, t114606)
}
