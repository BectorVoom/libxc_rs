//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 859/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk859<F: Float>(t55571: F, t8657: F, t1873: F, t23917: F, t3941: F, t6534: F, t7056: F, t45560: F, t20173: F, t31814: F, t31817: F, t1874: F, t91854: F, t23938: F, t6525: F, t2311: F) -> (F, F, F, F, F, F, F, F, F) {
    let t114517 = 27.0 * t55571 * t8657;
    let t114520 = 27.0 * t3941 * t23917 * t1873;
    let t114525 = 54.0 * t3941 * t7056 * t6534;
    let t114527 = 27.0 * t45560 * t8657;
    let t114529 = 54.0 * t20173 * t31814;
    let t114531 = 54.0 * t20173 * t31817;
    let t114541 = 4.0 * t91854 * t1874;
    let t114543 = 4.0 * t23938 * t6525;
    let t114552 = t2311 * t1873;
    (t114517, t114520, t114525, t114527, t114529, t114531, t114541, t114543, t114552)
}
