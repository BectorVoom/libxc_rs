//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1198/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1198<F: Float>(t3683: F, t580: F, t823: F, t10662: F, t19671: F, t8096: F, t19818: F, t1398: F, t1991: F, t10552: F, t30: F, t2133: F, t17930: F, t2: F, t2436: F, t555: F, t821: F) -> (F, F, F, F, F, F, F, F) {
    let t64260 = t823 * t580 * t3683;
    let t64263 = t19671 * t10662;
    let t64266 = t8096 * t580;
    let t64267 = t64266 * t19818;
    let t64273 = t1991 * t1398;
    let t64292 = t30 * t10552;
    let t64296 = t1398 * t2133;
    let t64297 = t17930 * t64296;
    let t64300 = t2436 * t2;
    let t64302 = t64300 * t555 * t821;
    (t64260, t64263, t64267, t64273, t64292, t64296, t64297, t64302)
}
