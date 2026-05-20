//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2224/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2224<F: Float>(t23237: F, t28276: F, t6552: F, t16662: F, t6553: F, t6554: F, t23164: F, t23204: F, t16968: F, t87052: F, t87053: F, t16887: F, t87057: F) -> (F, F, F, F, F) {
    let t98315 = t6552 * t23237 * t28276;
    let t98319 = t6552 * t6553 * t6554 * t16662;
    let t98322 = t23164 * t23204 * t28276;
    let t98325 = t87052 * t87053 * t16968;
    let t98328 = t87057 * t87053 * t16887;
    (t98315, t98319, t98322, t98325, t98328)
}
