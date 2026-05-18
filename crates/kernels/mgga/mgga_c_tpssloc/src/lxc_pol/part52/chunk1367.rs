//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1367/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1367<F: Float>(t33746: F, t6880: F, t1458: F, t7263: F, t1874: F, t2113: F, t4072: F, t33690: F, t6525: F, t31832: F, t7756: F, t119931: F, t2108: F, t2240: F) -> (F, F, F, F, F, F, F, F) {
    let t122914 = t33746 * t6880;
    let t122917 = t7263 * t1458;
    let t122918 = t122917 * t1874;
    let t122920 = t2113 * t4072;
    let t122921 = t122920 * t1874;
    let t122923 = t33690 * t6525;
    let t122925 = t31832 * t7756;
    let t122941 = t2240 * t119931 * t2108;
    (t122914, t122917, t122918, t122920, t122921, t122923, t122925, t122941)
}
