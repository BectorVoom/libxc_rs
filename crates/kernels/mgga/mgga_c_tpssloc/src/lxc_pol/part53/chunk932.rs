//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 932/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk932<F: Float>(t111: F, t7222: F, t25: F, t40772: F, t1519: F, t213: F, t225: F, t794: F, t214: F, t4265: F, t28: F, t1834: F) -> (F, F, F, F, F, F, F) {
    let t84033 = t7222 * t111;
    let t86716 = t40772 * t25;
    let t86873 = t213 * t1519 * t225;
    let t86893 = t794 * t1519;
    let t87782 = t214 * t4265;
    let t89953 = t40772 * t28;
    let t90544 = t794 * t1834;
    (t84033, t86716, t86873, t86893, t87782, t89953, t90544)
}
