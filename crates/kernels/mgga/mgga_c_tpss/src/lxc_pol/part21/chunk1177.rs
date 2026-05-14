//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1177/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1177<F: Float>(t33: F, t259: F, t479: F, t18230: F, t1749: F, t18278: F, t1992: F, t5686: F, t57: F, t581: F, t18238: F, t508: F, t5753: F, t5709: F, t1760: F, t3202: F, t9895: F, t1778: F, t5706: F, t5758: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t18279 = piecewise3(t480, 0.0, t18230);
    let t18286 = piecewise3(t386, t18278, t18279 * t57 / 2.0 - t5686 * t581 - t1749 * t1992 / 2.0);
    let t18287 = t18238 + t18286;
    let t18289 = t508 * t5753;
    let t18290 = t18289 * t5709;
    let t18292 = 6.0 * t1760 * t18290;
    let t18295 = t9895 * t3202;
    let t18296 = t1778 * t18295;
    let t18298 = 2.0 * t1760 * t18296;
    let t18304 = 2.0 * t5706 * t5758;
    (t18279, t18287, t18289, t18290, t18292, t18295, t18296, t18298, t18304)
}
