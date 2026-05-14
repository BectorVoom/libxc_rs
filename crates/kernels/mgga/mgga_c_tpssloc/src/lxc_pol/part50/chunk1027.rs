//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1027/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1027<F: Float>(t30689: F, t6562: F, t794: F, t22690: F, t23171: F, t30676: F, t30725: F, t814: F, t23012: F, t8332: F, t8336: F, t225: F, t30732: F, t40772: F, t8369: F, t2752: F, t30752: F) -> (F, F, F, F, F, F, F, F) {
    let t112997 = t6562 * t794 * t30689;
    let t113005 = 0.16449340668482264365e-1 * t23171 * t22690 * t30676;
    let t113016 = t814 * t30725;
    let t113038 = 0.12793931631041761173e0 * t23012 * t8332;
    let t113045 = 0.12793931631041761173e0 * t23012 * t8336;
    let t113053 = t30732 * t225;
    let t113082 = t8369 * t40772;
    let t113111 = t30752 * t2752;
    (t112997, t113005, t113016, t113038, t113045, t113053, t113082, t113111)
}
