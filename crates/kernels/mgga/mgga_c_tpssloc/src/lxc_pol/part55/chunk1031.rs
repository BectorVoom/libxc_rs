//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1031/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1031<F: Float>(t225: F, t30732: F, t40772: F, t8369: F, t2752: F, t30752: F, t10143: F, t8365: F, t193: F, t201: F, t79: F, t8306: F, t22642: F, t22643: F, t8458: F, t2006: F, t212: F, t6890: F) -> (F, F, F, F, F, F, F, F, F) {
    let t113053 = t30732 * t225;
    let t113082 = t8369 * t40772;
    let t113111 = t30752 * t2752;
    let t113117 = t8365 * t10143;
    let t113131 = t193 * t201 * t8365;
    let t113135 = t193 * t201 * t8369;
    let t113875 = t8306 * t79;
    let t113934 = 0.16449340668482264365e-1 * t22642 * t22643 * t8458;
    let t113941 = 0.16449340668482264365e-1 * t22642 * t212 * t2006 * t6890;
    (t113053, t113082, t113111, t113117, t113131, t113135, t113875, t113934, t113941)
}
