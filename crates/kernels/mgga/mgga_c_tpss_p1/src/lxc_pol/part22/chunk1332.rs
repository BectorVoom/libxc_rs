//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1332/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1332<F: Float>(t4425: F, t60738: F, t1630: F, t60730: F, t18436: F, t4409: F, t12996: F, t5716: F, t12978: F, t18454: F, t12982: F, t12986: F) -> (F, F, F, F, F, F, F) {
    let t65561 = t60738 * t4425;
    let t65567 = t60730 * t1630;
    let t65570 = t18436 * t4409;
    let t65572 = t5716 * t12996;
    let t65574 = t18454 * t12978;
    let t65576 = t18454 * t12982;
    let t65578 = t18454 * t12986;
    (t65561, t65567, t65570, t65572, t65574, t65576, t65578)
}
