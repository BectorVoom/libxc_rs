//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 894/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk894<F: Float>(t1512: F, t9674: F, t2638: F, t4166: F, t831: F, t4250: F, t9638: F, t4240: F, t4191: F, t2697: F, t4261: F, t820: F, t9645: F, t1484: F, t828: F, t1516: F, t9993: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13277 = 7.0 / 2304.0 * t9674 * t1512;
    let t13278 = t4166 * t2638;
    let t13280 = 7.0 / 2304.0 * t13278 * t831;
    let t13287 = 7.0 / 576.0 * t9638 * t4250;
    let t13320 = 7.0 / 2304.0 * t9638 * t4240;
    let t13330 = 7.0 / 576.0 * t9638 * t4191;
    let t13345 = 7.0 / 576.0 * t2697 * t4261;
    let t13350 = t9645 * t820;
    let t13351 = t1484 * t828;
    let t13359 = 7.0 / 576.0 * t9993 * t1516;
    (t13277, t13278, t13280, t13287, t13320, t13330, t13345, t13350, t13351, t13359)
}
