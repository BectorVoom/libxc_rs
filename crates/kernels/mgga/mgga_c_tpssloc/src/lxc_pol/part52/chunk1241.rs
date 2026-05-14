//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1241/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1241<F: Float>(t116326: F, t116328: F, t123261: F, t123280: F, t123292: F, t123304: F, t123313: F, t123319: F, t123322: F, t1398: F, t1404: F, t1852: F, t2023: F, t2029: F, t2170: F, t2174: F, t26510: F, t26555: F, t27908: F, t27930: F, t3: F, t31949: F, t33762: F, t5364: F, t5381: F, t580: F, t7003: F, t7426: F, t7759: F, t8119: F, t8693: F, t8702: F) -> (F,) {
    let t123325 = t3 * t123261 * t580 + t2170 * t26555 + t7759 * t7426 + t116328 + t1852 * t31949 + t7003 * t8119 + t116326 + t1398 * (t123280 + t123292 + t123304 + t123313) + t2023 * t27930 + t26510 * t2174 + t123319 + t33762 * t1404 + t27908 * t2029 + t123322 + t8693 * t5381 + t5364 * t8702;
    (t123325,)
}
