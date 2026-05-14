//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 858/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk858<F: Float>(t31193: F, t6347: F, t6637: F, t6888: F, t120492: F, t1799: F, t22685: F, t6330: F, t120490: F, t1992: F, t550: F, t6976: F, t97189: F, t97181: F, t120437: F, t1825: F, t22633: F) -> (F, F, F, F, F, F, F) {
    let t127371 = 0.3289868133696452873e-1 * t6888 * t6637 * t31193 * t6347;
    let t127375 = 0.6579736267392905746e-1 * t6888 * t6637 * t120492 * t1799;
    let t127381 = 0.9869604401089358619e-1 * t22685 * t6637 * t31193 * t6330;
    let t127382 = 0.3289868133696452873e-1 * t120490;
    let t127386 = 0.3289868133696452873e-1 * t1992 * t6976 * t97189 * t550;
    let t127391 = 0.16449340668482264365e-1 * t1992 * t6976 * t97181 * t550;
    let t127402 = 0.6579736267392905746e-1 * t22633 * t6976 * t120437 * t1825;
    (t127371, t127375, t127381, t127382, t127386, t127391, t127402)
}
