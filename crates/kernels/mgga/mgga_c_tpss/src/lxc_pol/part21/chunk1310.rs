//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1310/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1310<F: Float>(t10502: F, t10514: F, t10552: F, t10897: F, t1364: F, t14076: F, t1692: F, t1713: F, t17929: F, t18043: F, t18047: F, t19670: F, t19798: F, t198: F, t19802: F, t207: F, t2116: F, t2428: F, t2433: F, t2439: F, t3552: F, t3610: F, t44170: F, t44470: F, t5586: F, t5590: F, t60951: F, t6149: F, t64236: F, t64248: F, t64277: F, t64296: F, t64305: F, t750: F, t821: F, t823: F) -> (F,) {
    let t64770 = t10502 * t10514;
    let t64808 = 3.0 * t2439 * t18043 * t1364 - t1692 * t19802 * t2428 - 12.0 * t19670 * t44170 + 12.0 * t17929 * t64770 + 2.0 * t1692 * t64305 * t2433 + 6.0 * t3552 * t6149 * t2116 - 2.0 * t1692 * t64277 * t821 + 6.0 * t2439 * t19798 * t750 - 3.0 * t2439 * t5590 * t64296 - 6.0 * t2439 * t5590 * t44470 + 6.0 * t2439 * t5586 * t3610 + t198 * t207 * t64236 * t823 - 6.0 * t2439 * t18047 * t14076 + 3.0 * t2439 * t1713 * t10552 - t1692 * t5590 * t10897 - 6.0 * t1692 * t60951 * t64248;
    (t64808,)
}
