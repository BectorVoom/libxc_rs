//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 716/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk716<F: Float>(t1888: F, t25284: F, t2647: F, t4282: F, t6646: F, t22986: F, t6547: F, t7529: F, t25249: F, t829: F, t22996: F, t4283: F, t1484: F, t23153: F, t6637: F, t6552: F) -> (F, F, F, F, F, F) {
    let t25285 = t1888 * t25284;
    let t25287 = t4282 * t2647;
    let t25288 = t6646 * t25287;
    let t25289 = t22986 * t25288;
    let t25293 = t6547 * t7529;
    let t25299 = t25249 * t829;
    let t25300 = t6646 * t25299;
    let t25301 = t22986 * t25300;
    let t25303 = t22996 * t4283;
    let t25304 = t1888 * t25303;
    let t25306 = t23153 * t1484;
    let t25307 = t6637 * t25306;
    let t25308 = t6552 * t25307;
    (t25285, t25289, t25293, t25301, t25304, t25308)
}
