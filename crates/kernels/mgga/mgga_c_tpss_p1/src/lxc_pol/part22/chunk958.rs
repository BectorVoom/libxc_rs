//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 958/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk958<F: Float>(t509: F, t526: F, t235: F, t72: F, t3342: F, t3350: F, t1242: F, t2376: F, t339: F, t1250: F, t3354: F, t1184: F, t3211: F) -> (F, F, F, F, F, F) {
    let t9984 = F::new(1.0) / t526 / t509;
    let t9986 = t235 * t9984 * t72;
    let t9991 = t3342 * t3350;
    let t9994 = t339 * t1242 * t2376;
    let t9995 = t9994 * t1250;
    let t9997 = t3342 * t3354;
    let t10016 = t3211 * t1184;
    (t9986, t9991, t9994, t9995, t9997, t10016)
}
