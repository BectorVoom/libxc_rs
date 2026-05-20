//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1026/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1026<F: Float>(t33231: F, t7458: F, t1873: F, t5449: F, t2040: F, t127553: F, t22574: F, t24432: F, t1442: F, t33553: F, t5457: F, t8595: F) -> (F, F, F, F, F, F) {
    let t128516 = F::new(4.0) * t7458 * t33231;
    let t128521 = t5449 * t1873;
    let t128523 = F::new(2.0) * t128521 * t2040;
    let t128535 = F::new(6.0) * t22574 * t24432 * t127553;
    let t128537 = F::new(2.0) * t1442 * t33553;
    let t128539 = F::new(2.0) * t5457 * t8595;
    (t128516, t128521, t128523, t128535, t128537, t128539)
}
