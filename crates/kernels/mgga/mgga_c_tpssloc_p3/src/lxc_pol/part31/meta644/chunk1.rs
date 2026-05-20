//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1915/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1915<F: Float>(t1888: F, t23270: F, t25044: F, t4300: F, t5527: F, t857: F, t25038: F, t865: F, t23035: F, t23237: F, t28298: F, t23204: F, t81640: F) -> (F, F, F, F) {
    let t98222 = t1888 * t23270 * t25044 * t4300;
    let t98224 = t857 * t5527;
    let t98227 = t25038 * t23270 * t98224 * t865;
    let t98234 = t23035 * t23237 * t28298;
    let t98237 = t81640 * t23204 * t28298;
    (t98222, t98227, t98234, t98237)
}
