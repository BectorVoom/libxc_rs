//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 757/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk757<F: Float>(t1155: F, t3403: F, t3439: F, t60: F, t461: F, t3448: F, t457: F, t974: F, t1229: F, t3247: F, t1215: F, t3508: F) -> (F, F, F, F, F, F, F) {
    let t4883 = t3403 * t1155;
    let t4899 = t60 * t3439;
    let t4900 = t4899 * t461;
    let t4908 = t3448 * t461;
    let t4934 = t974 * t457;
    let t4972 = t1229 * t3247;
    let t4978 = t3508 * t1215;
    (t4883, t4899, t4900, t4908, t4934, t4972, t4978)
}
