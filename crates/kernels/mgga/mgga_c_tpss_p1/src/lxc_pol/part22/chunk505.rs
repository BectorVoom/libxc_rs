//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 505/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk505<F: Float>(t2025: F, t38: F, t45: F, t606: F, t78: F, t57: F, t610: F, t81: F, t1985: F, t1992: F, t608: F, t612: F) -> (F, F, F, F, F, F) {
    let t2026 = t38 * t2025;
    let t2031 = t606 * t45;
    let t2033 = F::cast_from(1.0_f64) / t78 / t2031;
    let t2038 = t610 * t57;
    let t2040 = F::cast_from(1.0_f64) / t81 / t2038;
    let t2045 = F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t2033 * t1985 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t608 * t1992 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t2040 * t1985 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t612 * t1992;
    (t2026, t2031, t2033, t2038, t2040, t2045)
}
