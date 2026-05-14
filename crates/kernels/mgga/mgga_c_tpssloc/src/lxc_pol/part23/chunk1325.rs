//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1325/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1325<F: Float>(t300: F, t78874: F, t78914: F, t78944: F, t79002: F, t78335: F, t78338: F, t78344: F, t78355: F, t78357: F, t78359: F, t78361: F, t78364: F, t78367: F, t78370: F, t78373: F) -> (F, F) {
    let t79005 = t300 * (t78874 + t78914 + t78944 + t79002);
    let t79006 = t78335 + t78338 - t78344 + t78355 - t78357 - t78359 + t78361 - t78364 - t78367 + t78370 + t78373 + t79005;
    (t79005, t79006)
}
