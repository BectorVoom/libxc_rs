//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1251/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1251<F: Float>(t22690: F, t23171: F, t30676: F, t30725: F, t814: F, t23012: F, t8332: F, t8336: F, t225: F, t30732: F, t40772: F, t8369: F) -> (F, F, F, F, F, F) {
    let t113005 = F::cast_from(0.16449340668482264365e-1_f64) * t23171 * t22690 * t30676;
    let t113016 = t814 * t30725;
    let t113038 = F::cast_from(0.12793931631041761173e0_f64) * t23012 * t8332;
    let t113045 = F::cast_from(0.12793931631041761173e0_f64) * t23012 * t8336;
    let t113053 = t30732 * t225;
    let t113082 = t8369 * t40772;
    (t113005, t113016, t113038, t113045, t113053, t113082)
}
