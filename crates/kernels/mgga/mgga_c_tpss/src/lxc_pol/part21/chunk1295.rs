//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1295/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1295<F: Float>(t198: F, t5659: F, t10552: F, t30: F, t1398: F, t2133: F, t17930: F, t2: F, t2436: F, t555: F, t821: F, t19816: F, t6148: F, t8096: F, t1692: F, t1713: F, t17929: F, t17931: F, t18053: F, t18059: F, t19670: F, t19802: F, t19810: F, t19819: F, t2439: F, t5590: F, t5591: F, t61264: F, t61269: F, t6153: F, t64256: F, t64260: F, t64263: F, t64267: F, t64273: F, t64277: F, t64284: F) -> (F, F, F, F, F) {
    let t64289 = t198 * t5659;
    let t64292 = t30 * t10552;
    let t64296 = t1398 * t2133;
    let t64297 = t17930 * t64296;
    let t64300 = t2436 * t2;
    let t64302 = t64300 * t555 * t821;
    let t64304 = 2.0 * t19816 * t64302;
    let t64305 = t6148 * t8096;
    let t64308 = -3.0 / 2.0 * t17929 * t64256 + 6.0 * t19670 * t64260 + 6.0 * t19670 * t64263 + 2.0 * t19816 * t64267 - t1692 * t61264 * t6153 / 2.0 - t1692 * t5590 * t64273 / 2.0 - t1692 * t64277 * t5591 - t1692 * t19802 * t18059 / 2.0 - 3.0 * t64284 * t17931 - 3.0 * t61269 * t19810 + 2.0 * t64289 * t19819 + 3.0 / 2.0 * t2439 * t1713 * t64292 - 3.0 / 2.0 * t17929 * t64297 - t64304 + t1692 * t64305 * t18053;
    (t64289, t64296, t64304, t64305, t64308)
}
