//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 985/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk985<F: Float>(t31193: F, t6347: F, t6637: F, t6888: F, t120492: F, t1799: F, t22685: F, t6330: F, t120490: F, t1992: F, t550: F, t6976: F, t97189: F) -> (F, F, F, F, F) {
    let t127371 = F::cast_from(0.3289868133696452873e-1_f64) * t6888 * t6637 * t31193 * t6347;
    let t127375 = F::cast_from(0.6579736267392905746e-1_f64) * t6888 * t6637 * t120492 * t1799;
    let t127381 = F::cast_from(0.9869604401089358619e-1_f64) * t22685 * t6637 * t31193 * t6330;
    let t127382 = F::cast_from(0.3289868133696452873e-1_f64) * t120490;
    let t127386 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t6976 * t97189 * t550;
    (t127371, t127375, t127381, t127382, t127386)
}
