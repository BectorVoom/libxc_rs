//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 599/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk599<F: Float>(t457: F, t974: F, t1229: F, t3247: F, t1215: F, t3508: F, t3242: F, t3584: F, t1932: F, t475: F, t671: F, t88: F, t193: F, t531: F, t533: F, t131: F, t3732: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4934 = t974 * t457;
    let t4972 = t1229 * t3247;
    let t4978 = t3508 * t1215;
    let t4987 = t3584 * t3242;
    let t5079 = t1932 * t1215 * t475;
    let t5113 = t88 * t671;
    let t5126 = t193 * t531;
    let t5160 = t193 * t533;
    let t5194 = t3732 * t131;
    (t4934, t4972, t4978, t4987, t5079, t5113, t5126, t5160, t5194)
}
