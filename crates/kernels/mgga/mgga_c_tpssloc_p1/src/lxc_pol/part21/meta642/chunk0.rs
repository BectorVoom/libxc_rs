//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2432/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2432<F: Float>(t10469: F, t990: F, t10471: F, t10875: F, t10468: F, t191: F, t349: F) -> (F, F, F, F, F) {
    let t42332 = t990 * t10469;
    let t42333 = t42332 * t10471;
    let t42334 = t42333 * t10875;
    let t42339 = F::cast_from(1.0_f64) / t10468 / t191;
    let t42340 = t349 * t42339;
    (t42332, t42333, t42334, t42339, t42340)
}
