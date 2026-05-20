//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1527/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1527<F: Float>(t16398: F, t5252: F, t3777: F, t5245: F, t1834: F, t3787: F, t225: F, t5319: F, t5217: F, t1390: F, t5356: F, t112: F, t5363: F) -> (F, F, F, F, F, F, F) {
    let t16400 = F::new(7.0) / F::new(1152.0) * t16398 * t5252;
    let t16401 = t3777 * t5245;
    let t16428 = t3787 * t1834;
    let t16439 = t5319 * t225;
    let t16460 = t5217 * t225;
    let t16497 = t5356 * t1390;
    let t16521 = t5363 * t112;
    (t16400, t16401, t16428, t16439, t16460, t16497, t16521)
}
