//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1072/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1072<F: Float>(t41666: F, t42308: F, t10468: F, t191: F, t349: F, t10471: F, t68: F) -> (F, F, F, F) {
    let t42309 = t42308 * t41666;
    let t42339 = 1.0 / t10468 / t191;
    let t42340 = t349 * t42339;
    let t42341 = t10471 * t68;
    (t42309, t42339, t42340, t42341)
}
