//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1050/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1050<F: Float>(t124728: F, t127720: F, t127722: F, t127726: F, t127728: F, t127730: F, t127736: F, t127738: F, t127742: F, t128289: F, t128298: F, t128300: F, t128302: F, t128303: F, t129015: F, t1459: F, t2040: F, t27863: F, t33746: F, t7796: F, t7806: F, t7941: F) -> F {
    let t130302 = -F::new(4.0) * t124728 * t1459 - F::new(4.0) * t129015 * t2040 - F::new(4.0) * t27863 * t7796 - F::new(4.0) * t27863 * t7806 + F::new(2.0) * t33746 * t7941 - t127720 - t127722 - t127726 - t127728 - t127730 + t127736 - t127738 - t127742 - t128289 - t128298 - t128300 - t128302 + t128303;
    t130302
}
