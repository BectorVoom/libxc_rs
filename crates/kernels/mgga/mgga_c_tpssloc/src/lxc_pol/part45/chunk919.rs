//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 919/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk919<F: Float>(t31688: F, t31693: F, t2303: F, t31691: F, t8513: F, t31687: F, t8515: F, t9231: F, t31019: F, t2240: F, t240: F, t8301: F, t113824: F, t113833: F, t113890: F, t113907: F, t115829: F, t115834: F, t31672: F, t31675: F, t39049: F, t8511: F, t8512: F) -> (F,) {
    let t115837 = t31688 * t31693;
    let t115842 = t8513 * t31691 * t2303;
    let t115846 = t9231 * t31687 * t8515;
    let t115853 = t31688 * t31019;
    let t115860 = 55.0 / 81.0 * t2240 * t8301 * t240 * t8515;
    let t115861 = 5.0 / 6.0 * t31675 * t113890 + 5.0 / 12.0 * t31675 * t115829 - 5.0 / 9.0 * t113824 * t115834 + 20.0 / 27.0 * t115837 - 5.0 / 18.0 * t8512 * t113907 - 5.0 / 36.0 * t8512 * t115842 + 10.0 / 27.0 * t115846 - 5.0 / 72.0 * t39049 * t8511 * t8515 - 5.0 / 36.0 * t31672 * t31019 + 10.0 / 27.0 * t115853 - 5.0 / 72.0 * t8512 * t113833 - t115860;
    (t115861,)
}
