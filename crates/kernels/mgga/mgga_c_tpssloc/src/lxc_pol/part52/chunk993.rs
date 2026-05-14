//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 993/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk993<F: Float>(t1339: F, t26301: F, t22827: F, t22833: F, t5259: F, t22759: F, t242: F, t1336: F, t5252: F, t5293: F, t5303: F, t1351: F, t16311: F, t3788: F, t6936: F, t16306: F, t550: F) -> (F, F, F, F, F, F, F, F) {
    let t26302 = t1339 * t26301;
    let t26303 = t22827 * t26302;
    let t26306 = t22833 * t5259;
    let t26308 = t22759 * t242;
    let t26309 = t1336 * t26308;
    let t26310 = t26309 * t5252;
    let t26312 = t22833 * t5293;
    let t26314 = t22833 * t5303;
    let t26318 = t16311 * t1351;
    let t26319 = t3788 * t26318;
    let t26320 = t6936 * t26319;
    let t26322 = t16306 * t550;
    (t26303, t26306, t26310, t26312, t26314, t26318, t26320, t26322)
}
