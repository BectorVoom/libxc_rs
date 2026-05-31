//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1360/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1360<F: Float>(t62711: F, t63998: F, t66423: F, t66427: F, t66429: F, t66434: F, t69989: F, t69991: F, t69993: F, t69995: F, t69997: F, t69999: F, t70001: F) -> F {
    let t72077 = F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t69989 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t69991 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t69993 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t69995 - t69997 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t69999 - F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t70001 - t62711 + t66423 + t66427 - t66429 - t66434 - t63998;
    t72077
}
