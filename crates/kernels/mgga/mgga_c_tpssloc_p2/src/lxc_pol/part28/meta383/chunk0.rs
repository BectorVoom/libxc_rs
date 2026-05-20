//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1471/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1471<F: Float>(t1706: F, t3428: F, t1184: F, t460: F, t4928: F, t4934: F, t1714: F, t3469: F, t1178: F, t12606: F, t1177: F, t135: F, t457: F) -> (F, F, F, F, F) {
    let t15265 = t1706 * t3428;
    let t15268 = t4928 * t1184 * t460;
    let t15269 = t4934 * t15268;
    let t15273 = t1714 * t3469 * t460;
    let t15274 = t4934 * t15273;
    let t15277 = t1178 * t12606;
    let t15278 = t1177 * t15277;
    let t15281 = t135 * t457;
    (t15265, t15269, t15274, t15278, t15281)
}
