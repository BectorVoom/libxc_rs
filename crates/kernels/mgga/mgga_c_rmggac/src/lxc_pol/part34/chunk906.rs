//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 906/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk906<F: Float>(t8264: F, t884: F, t8946: F, t70549: F, t638: F, t639: F, t702: F, t8849: F, t70517: F, t70524: F, t72117: F, t72119: F, t76515: F, t78536: F, t78540: F, t78544: F, t78545: F, t78546: F, t78547: F, t78548: F, t78551: F, t78553: F) -> (F,) {
    let t78556 = 0.11974241701863808564e0 * t884 * t8264 * t8946;
    let t78557 = 0.638468998399467591e-4 * t70549;
    let t78560 = t638 * t639 * t8849 * t702;
    let t78561 = 0.15243824895787514157e-3 * t78560;
    let t78562 = -t78536 + t78540 - t78544 + t78545 + t78546 - t72117 - t78547 + t72119 + t78548 + 0.6505345598561924296e-5 * t70517 + 0.6505345598561924296e-5 * t70524 - t78551 + t78553 - t78556 - t76515 - t78557 + t78561;
    (t78562,)
}
