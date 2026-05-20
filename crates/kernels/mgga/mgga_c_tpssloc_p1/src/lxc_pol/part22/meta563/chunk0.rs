//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2067/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2067<F: Float>(t10471: F, t42332: F, t10875: F, t10468: F, t191: F, t349: F) -> (F, F, F, F) {
    let t42333 = t42332 * t10471;
    let t42334 = t42333 * t10875;
    let t42339 = F::new(1.0) / t10468 / t191;
    let t42340 = t349 * t42339;
    (t42333, t42334, t42339, t42340)
}
