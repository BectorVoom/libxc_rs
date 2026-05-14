//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1021/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1021<F: Float>(t246: F, t4758: F, t4778: F, t768: F, t10845: F, t10884: F, t1379: F, t14179: F, t14210: F, t14298: F, t14349: F, t14375: F, t14388: F, t220: F, t229: F, t2415: F, t339: F, t3630: F, t3665: F, t3703: F, t3704: F, t3713: F, t3716: F, t4716: F, t4759: F, t4764: F, t783: F, t813: F, t8361: F) -> (F,) {
    let t14401 = t246 * t4758;
    let t14418 = t768 * t4778;
    let t14423 = -6.0 * t10845 * t14210 * t14375 - 2.0 * t10884 * t1379 * t339 + 4.0 * t14179 * t3703 * t3704 - t14298 * t339 * t813 + t14349 * t220 * t229 + 6.0 * t14375 * t3630 * t3703 - t14375 * t3713 * t783 + 4.0 * t14388 * t3630 * t3703 - 2.0 * t14388 * t3713 * t783 + 2.0 * t14401 * t3630 * t3703 - t14401 * t3713 * t783 - t14418 * t339 * t783 - t2415 * t339 * t4759 - t2415 * t339 * t4764 - 2.0 * t339 * t3665 * t3716 + 2.0 * t339 * t4716 * t8361 - 2.0 * t3665 * t3704 * t3713;
    (t14423,)
}
