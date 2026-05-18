//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1339/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1339<F: Float>(t6604: F, t9971: F, t1888: F, t81672: F, t9975: F, t22996: F, t2632: F, t23110: F, t23180: F, t23185: F, t206: F, t22723: F, t268: F) -> (F, F, F, F) {
    let t82018 = t6604 * t9971;
    let t82021 = t1888 * t82018 * t81672 * t9975;
    let t82025 = t1888 * t22996 * t81672 * t2632;
    let t82028 = t23185 * t23110 * t23180;
    let t82031 = t22723 * t206 * t268;
    (t82021, t82025, t82028, t82031)
}
