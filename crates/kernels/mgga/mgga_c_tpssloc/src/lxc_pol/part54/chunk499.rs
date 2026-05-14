//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 499/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk499<F: Float>(t1284: F, t67: F, t758: F, t2225: F, t522: F, t2221: F, t2516: F, t521: F, t17: F, t750: F, t1285: F, t592: F, t1287: F, t588: F, t248: F, t2691: F, t557: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3814 = t1284 * t67;
    let t3815 = t3814 * t758;
    let t3819 = 20.0 * t2225 * t522;
    let t3821 = 12.0 * t2221 * t522;
    let t3824 = t521 * t2516;
    let t3825 = t17 * t3824;
    let t3826 = t1284 * t750;
    let t3827 = t17 * t3826;
    let t3829 = t592 * t1285;
    let t3832 = 8.0 * t592 * t1287;
    let t3833 = t588 * t1285;
    let t3862 = t2691 * t557 * t248;
    (t3815, t3819, t3821, t3825, t3827, t3829, t3832, t3833, t3862)
}
