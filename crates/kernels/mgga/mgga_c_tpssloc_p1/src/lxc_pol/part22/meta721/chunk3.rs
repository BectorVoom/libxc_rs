//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2347/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2347<F: Float>(t12984: F, t12998: F, t5544: F, t686: F, t20933: F, t2563: F, t20923: F, t41011: F, t118: F, t20756: F, t41170: F, t794: F) -> (F, F, F, F) {
    let t68110 = t12998 * t686 * t12984 * t5544;
    let t68116 = t2563 * t20933;
    let t68118 = t41011 * t20923;
    let t68122 = t41170 * t118 * t794 * t20756;
    (t68110, t68116, t68118, t68122)
}
