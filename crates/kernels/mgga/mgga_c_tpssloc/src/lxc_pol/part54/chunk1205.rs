//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1205/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1205<F: Float>(t23938: F, t7461: F, t26977: F, t25980: F, t7042: F, t33553: F, t650: F, t31759: F, t7685: F, t31300: F, t91655: F, t2018: F, t22574: F, t24432: F, t5187: F, t24995: F, t37790: F, t5308: F) -> (F, F, F, F, F, F, F, F) {
    let t120964 = 2.0 * t23938 * t7461;
    let t120966 = 2.0 * t26977 * t7461;
    let t120968 = 2.0 * t7042 * t25980;
    let t120973 = t650 * t33553;
    let t120975 = 3.0 * t7685 * t31759;
    let t120979 = 3.0 * t91655 * t31300;
    let t120986 = 3.0 * t22574 * t24432 * t2018 * t5187;
    let t120991 = 6.0 * t24995 * t37790 * t5308;
    (t120964, t120966, t120968, t120973, t120975, t120979, t120986, t120991)
}
