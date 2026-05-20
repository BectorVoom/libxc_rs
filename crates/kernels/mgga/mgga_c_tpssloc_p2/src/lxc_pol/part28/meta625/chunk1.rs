//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1951/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1951<F: Float>(t1324: F, t254: F, t22724: F, t26344: F, t22643: F, t7691: F, t81195: F, t1388: F, t25988: F, t1845: F, t3719: F, t22573: F, t7684: F) -> (F, F, F, F, F, F) {
    let t91505 = t1324 * t254;
    let t91531 = t22724 * t26344;
    let t91548 = t81195 * t22643 * t7691;
    let t91565 = t25988 * t1388;
    let t91603 = t1845 * t3719;
    let t91655 = t7684 * t22573;
    (t91505, t91531, t91548, t91565, t91603, t91655)
}
