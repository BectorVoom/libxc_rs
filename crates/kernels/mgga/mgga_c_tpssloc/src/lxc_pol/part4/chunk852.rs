//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 852/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk852<F: Float>(t13257: F, t812: F, t4184: F, t242: F, t9972: F, t2639: F, t4236: F, t1512: F, t9674: F, t2638: F, t4166: F, t831: F, t4250: F, t9638: F, t4240: F, t4191: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13258 = t812 * t13257;
    let t13260 = 7.0 / 1152.0 * t13258 * t4184;
    let t13261 = t9972 * t242;
    let t13262 = t812 * t13261;
    let t13275 = 7.0 / 2304.0 * t2639 * t4236;
    let t13277 = 7.0 / 2304.0 * t9674 * t1512;
    let t13278 = t4166 * t2638;
    let t13280 = 7.0 / 2304.0 * t13278 * t831;
    let t13287 = 7.0 / 576.0 * t9638 * t4250;
    let t13320 = 7.0 / 2304.0 * t9638 * t4240;
    let t13330 = 7.0 / 576.0 * t9638 * t4191;
    (t13260, t13262, t13275, t13277, t13278, t13280, t13287, t13320, t13330)
}
