//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1318/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1318<F: Float>(t1398: F, t2433: F, t64247: F, t17930: F, t35525: F, t3683: F, t580: F, t823: F, t10662: F, t19671: F, t8096: F, t19818: F) -> (F, F, F, F, F, F) {
    let t64248 = t1398 * t2433;
    let t64249 = t64247 * t64248;
    let t64256 = t17930 * t35525;
    let t64260 = t823 * t580 * t3683;
    let t64263 = t19671 * t10662;
    let t64266 = t8096 * t580;
    let t64267 = t64266 * t19818;
    (t64248, t64249, t64256, t64260, t64263, t64267)
}
