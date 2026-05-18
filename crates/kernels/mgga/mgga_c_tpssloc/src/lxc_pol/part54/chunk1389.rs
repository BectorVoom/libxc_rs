//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1389/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1389<F: Float>(t121593: F, t121606: F, t121495: F, t25038: F, t25248: F, t776: F, t114649: F, t114752: F, t118756: F, t118760: F, t118764: F, t118767: F, t121553: F, t121560: F, t121563: F, t121574: F, t1510: F, t226: F, t235: F, t2617: F, t31394: F, t31395: F, t33388: F, t4162: F, t4166: F, t4182: F, t4234: F, t4281: F, t4291: F, t812: F, t829: F, t8560: F) -> (F, F) {
    let t121607 = t121593 + t121606;
    let t121612 = t25038 * t25248 * t121495 * t776;
    let t121614 = F::new(2.0) * t4281 * t121553 * t4182 + t4162 * t8560 - t118756 - F::new(0.82246703342411321825e-2) * t121560 - F::new(0.82246703342411321825e-2) * t121563 - t812 * t31394 * t4234 - t2617 * t33388 - t812 * t114649 * t1510 + F::new(0.19190897446562641759e-1) * t114752 - t4166 * t31395 - t118760 - t118764 + t118767 - t4291 * t121553 * t829 - F::new(0.19190897446562641759e-1) * t121574 + t226 * t235 * t121607 + F::new(0.49348022005446793095e-1) * t121612;
    (t121607, t121614)
}
