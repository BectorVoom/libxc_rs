//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1234/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1234<F: Float>(t10200: F, t10214: F, t10219: F, t10235: F, t10278: F, t2960: F, t2986: F, t340: F, t343: F, t39097: F, t39110: F, t42968: F, t42974: F, t42976: F, t42985: F, t43000: F, t43012: F, t43019: F, t43028: F, t973: F, t974: F, t977: F, t978: F) -> (F,) {
    let t43034 = 0.19753086419753086419e-2 * t42968 - 0.92181069958847736624e-2 * t2960 * t10219 + 0.11522633744855967078e-2 * t42974 - 0.1037037037037037037e-1 * t973 * t10214 * t42976 * t39097 + 0.27777777777777777777e-3 * t973 * t977 * t978 * t39110 - 0.44444444444444444444e-2 * t2986 * t10235 * t42985 - 0.83333333333333333332e-3 * t973 * t974 * t340 * (t43000 + t43012) * t343 - 0.24999999999999999999e-2 * t973 * t974 * t340 * t43019 * t343 - 0.17777777777777777777e-1 * t2960 * t10200 + 0.22222222222222222221e-2 * t43028 - 0.66666666666666666664e-2 * t973 * t977 * t10278 * t39097;
    (t43034,)
}
