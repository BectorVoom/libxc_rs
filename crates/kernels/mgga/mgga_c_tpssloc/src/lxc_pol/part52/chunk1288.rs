//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1288/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1288<F: Float>(t1888: F, t232: F, t6646: F, t87620: F, t23110: F, t23185: F, t32822: F, t112990: F, t112995: F, t113005: F, t118730: F, t118735: F, t118736: F, t118737: F, t118739: F, t118743: F, t118745: F, t118751: F, t118756: F, t118760: F, t1499: F, t2617: F, t30695: F, t30726: F, t32831: F, t4162: F, t4166: F, t8360: F) -> F {
    let t118764 = F::new(0.16449340668482264365e-1) * t1888 * t6646 * t87620 * t232;
    let t118766 = t23185 * t23110 * t32822;
    let t118767 = F::new(0.82246703342411321825e-2) * t118766;
    let t118768 = t1499 * t30726 - t2617 * t32831 - t30695 * t4166 + t4162 * t8360 + t112990 + t112995 - t113005 + t118730 - t118735 - t118736 - t118737 - t118739 + t118743 + t118745 - t118751 - t118756 - t118760 - t118764 + t118767;
    t118768
}
