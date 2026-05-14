//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1278/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1278<F: Float>(t17930: F, t35525: F, t3683: F, t580: F, t823: F, t10662: F, t19671: F, t8096: F, t19818: F, t1398: F, t1991: F, t19797: F, t2436: F, t198: F, t206: F, t6148: F) -> (F, F, F, F, F, F, F) {
    let t64256 = t17930 * t35525;
    let t64260 = t823 * t580 * t3683;
    let t64263 = t19671 * t10662;
    let t64266 = t8096 * t580;
    let t64267 = t64266 * t19818;
    let t64273 = t1991 * t1398;
    let t64277 = t19797 * t2436;
    let t64284 = t198 * t206 * t6148;
    (t64256, t64260, t64263, t64267, t64273, t64277, t64284)
}
