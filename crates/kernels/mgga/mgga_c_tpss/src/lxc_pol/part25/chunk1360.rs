//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1360/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1360<F: Float>(t62711: F, t63998: F, t66423: F, t66427: F, t66429: F, t66434: F, t69989: F, t69991: F, t69993: F, t69995: F, t69997: F, t69999: F, t70001: F) -> F {
    let t72077 = F::new(5.0) / F::new(96.0) * t69989 + F::new(5.0) / F::new(192.0) * t69991 + F::new(7.0) / F::new(1152.0) * t69993 + F::new(7.0) / F::new(1152.0) * t69995 - t69997 / F::new(768.0) - F::new(7.0) / F::new(576.0) * t69999 - F::new(5.0) / F::new(32.0) * t70001 - t62711 + t66423 + t66427 - t66429 - t66434 - t63998;
    t72077
}
