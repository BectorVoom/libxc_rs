//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1440/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1440<F: Float>(t1992: F, t22635: F, t26990: F, t115332: F, t1985: F, t7700: F, t120350: F, t120375: F, t113967: F, t113988: F, t114000: F, t115447: F, t120342: F, t120344: F, t120348: F, t120357: F, t120363: F, t120366: F, t120369: F, t120372: F, t120377: F, t120379: F, t120381: F, t120383: F) -> (F, F, F) {
    let t122399 = t1992 * t22635 * t26990;
    let t122406 = t1985 * t115332 * t7700;
    let t122411 = F::new(7.0) / F::new(1152.0) * t120350;
    let t122417 = F::new(7.0) / F::new(288.0) * t120375;
    let t122423 = -t120342 / F::new(768.0) - t120344 / F::new(768.0) - t120348 / F::new(768.0) + t122411 + F::new(5.0) / F::new(192.0) * t120357 + t113967 + F::new(0.26915170729426927235e-3) * t120363 - t115447 + F::new(0.96894614625936938046e-2) * t120366 + F::new(0.96894614625936938046e-2) * t120369 - F::new(0.16149102437656156341e-2) * t120372 + t113988 + t122417 - t120377 / F::new(192.0) - t120379 / F::new(192.0) - t120381 / F::new(192.0) + F::new(0.67826230238155856632e-1) * t120383 + F::new(0.67826230238155856634e-1) * t114000;
    (t122399, t122406, t122423)
}
