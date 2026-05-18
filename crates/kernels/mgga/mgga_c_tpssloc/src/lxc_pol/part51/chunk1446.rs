//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1446/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1446<F: Float>(t33285: F, t6883: F, t33284: F, t6897: F, t794: F, t1992: F, t22897: F, t27075: F, t27078: F, t6976: F, t115430: F, t115433: F, t115435: F, t115439: F, t120483: F, t120487: F, t120491: F, t120496: F, t120502: F, t122439: F, t122471: F, t1332: F, t1352: F, t33291: F, t5344: F, t544: F, t553: F) -> F {
    let t122503 = t6883 * t33285;
    let t122507 = t6897 * t794 * t33284;
    let t122510 = t1992 * t22897 * t27075;
    let t122513 = t1992 * t6976 * t27078;
    let t122515 = -F::new(0.19190897446562641759e-1) * t115430 + t115433 + t115435 - t5344 * t122471 * t1352 + t544 * t553 * t122439 - t120483 - t120487 + t120491 - F::new(0.41123351671205660912e-2) * t115439 - F::new(0.19190897446562641759e-1) * t122503 - t120496 + t1332 * t33291 - F::new(0.41123351671205660912e-2) * t122507 + F::new(0.16449340668482264365e-1) * t122510 - F::new(0.82246703342411321825e-2) * t122513 - t120502;
    t122515
}
