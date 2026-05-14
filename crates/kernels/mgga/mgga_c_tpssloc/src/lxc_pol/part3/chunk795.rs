//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 795/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk795<F: Float>(t1352: F, t5248: F, t5249: F, t120: F, t1799: F, t3805: F, t1831: F, t3866: F, t1307: F, t3870: F, t820: F, t1367: F, t5187: F, t1341: F, t1363: F, t3781: F, t3783: F, t3800: F, t3803: F, t3864: F, t3867: F, t5259: F, t5289: F) -> (F, F, F, F, F, F, F) {
    let t5293 = t5248 * t5249 * t1352;
    let t5301 = t120 * t1799;
    let t5303 = t3805 * t5301 * t1352;
    let t5306 = t3866 * t1831;
    let t5308 = t1799 * t1307;
    let t5310 = t3870 * t820 * t5308;
    let t5314 = t1367 * t820 * t5187;
    let t5317 = t3803 * t5259 / 768.0 - t1341 * t5289 / 3072.0 - t3803 * t5293 / 3072.0 - 7.0 / 4608.0 * t3781 + 7.0 / 4608.0 * t3800 + t3864 + 7.0 / 1152.0 * t3867 - t3783 * t1831 / 768.0 + t3803 * t5303 / 768.0 + 7.0 / 1152.0 * t5306 + 5.0 / 768.0 * t1363 * t5310 - t1363 * t5314 / 768.0;
    (t5293, t5301, t5303, t5308, t5310, t5314, t5317)
}
