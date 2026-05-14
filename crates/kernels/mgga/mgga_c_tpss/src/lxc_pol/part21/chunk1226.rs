//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1226/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1226<F: Float>(t1395: F, t5562: F, t5572: F, t1702: F, t3721: F, t1379: F, t818: F, t18007: F, t5570: F, t811: F, t1706: F) -> (F, F, F, F, F, F, F, F) {
    let t19753 = t5562 * t1395;
    let t19754 = t5572 * t19753;
    let t19757 = t1702 * t3721;
    let t19758 = t5572 * t19757;
    let t19762 = t1379 * t818;
    let t19763 = t18007 * t19762;
    let t19766 = t5570 * t811;
    let t19767 = t1706 * t19766;
    (t19753, t19754, t19757, t19758, t19762, t19763, t19766, t19767)
}
