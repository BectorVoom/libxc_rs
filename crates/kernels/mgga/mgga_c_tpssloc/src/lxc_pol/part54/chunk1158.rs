//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1158/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1158<F: Float>(t31394: F, t829: F, t235: F, t31361: F, t226: F, t30675: F, t30680: F, t30683: F, t30688: F, t30692: F, t31375: F, t31379: F, t31383: F, t31387: F, t31391: F, t808: F, t812: F, t8560: F) -> (F, F, F) {
    let t31395 = t31394 * t829;
    let t31397 = t235 * t31361;
    let t31399 = -t30675 - t30680 - t30683 - t30688 + t30692 - t31375 - F::new(0.16449340668482264365e-1) * t31379 - t31383 - F::new(0.82246703342411321825e-2) * t31387 + F::new(0.82246703342411321825e-2) * t31391 + t808 * t8560 - t812 * t31395 + t226 * t31397;
    (t31395, t31397, t31399)
}
