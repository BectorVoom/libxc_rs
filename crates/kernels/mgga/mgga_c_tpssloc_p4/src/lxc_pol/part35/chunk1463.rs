//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1463/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1463<F: Float>(t104990: F, t106968: F, t106974: F, t106978: F, t107492: F, t107496: F, t107499: F, t107507: F, t107509: F, t107512: F, t107515: F, t107519: F, t107521: F, t107523: F, t107525: F, t107527: F, t107530: F, t107533: F, t107539: F, t109976: F, t1459: F, t574: F) -> F {
    let t109980 = -F::new(6.0) * t104990 * t1459 + t109976 * t574 + t106968 - t106974 + t106978 + t107492 - t107496 - t107499 - t107507 - t107509 + t107512 + t107515 - t107519 - t107521 - t107523 - t107525 - t107527 - t107530 - t107533 - t107539;
    t109980
}
