//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2227/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2227<F: Float>(t17037: F, t1888: F, t22996: F, t232: F, t58204: F, t6646: F, t2632: F, t58166: F, t28423: F, t6579: F, t28427: F, t1902: F, t5611: F) -> (F, F, F, F, F, F) {
    let t98478 = t1888 * t22996 * t17037;
    let t98482 = t1888 * t6646 * t58204 * t232;
    let t98486 = t1888 * t22996 * t58166 * t2632;
    let t98488 = t6579 * t28423;
    let t98490 = t6579 * t28427;
    let t98494 = t1902 * t5611;
    (t98478, t98482, t98486, t98488, t98490, t98494)
}
