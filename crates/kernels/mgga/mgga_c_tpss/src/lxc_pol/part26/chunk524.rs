//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 524/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk524<F: Float>(t234: F, t64: F, t45: F, t606: F, t78: F, t57: F, t610: F, t81: F, t116: F, t623: F) -> (F, F, F, F, F, F, F) {
    let t2023 = t64 * t234;
    let t2024 = 88.0 / 9.0 * t2023;
    let t2031 = t606 * t45;
    let t2033 = 1.0 / t78 / t2031;
    let t2038 = t610 * t57;
    let t2040 = 1.0 / t81 / t2038;
    let t2056 = t623 * t116;
    (t2023, t2024, t2031, t2033, t2038, t2040, t2056)
}
