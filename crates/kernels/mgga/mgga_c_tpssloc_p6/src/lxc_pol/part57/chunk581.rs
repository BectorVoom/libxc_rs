//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 581/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk581<F: Float>(t1484: F, t6638: F, t6637: F, t6552: F, t232: F, t4282: F, t6646: F, t1888: F, t1519: F, t1894: F, t214: F, t1880: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7520 = t6638 * t1484;
    let t7521 = t6637 * t7520;
    let t7522 = t6552 * t7521;
    let t7524 = t4282 * t232;
    let t7525 = t6646 * t7524;
    let t7526 = t1888 * t7525;
    let t7528 = t1894 * t1519;
    let t7529 = t214 * t7528;
    let t7530 = t1880 * t7529;
    (t7520, t7521, t7522, t7524, t7525, t7526, t7528, t7529, t7530)
}
