//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2125/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2125<F: Float>(t1568: F, t2886: F, t2860: F, t4408: F, t10770: F, t1561: F, t2791: F, t4351: F, t10660: F, t1543: F, t10756: F, t300: F) -> (F, F, F, F, F, F) {
    let t49422 = t2886 * t1568;
    let t49427 = t4408 * t2860;
    let t49430 = t1561 * t10770;
    let t49486 = t4351 * t2791;
    let t49489 = t1543 * t10660;
    let t49513 = t300 * t10756;
    (t49422, t49427, t49430, t49486, t49489, t49513)
}
