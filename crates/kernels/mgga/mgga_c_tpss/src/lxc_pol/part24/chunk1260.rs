//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1260/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1260<F: Float>(t4466: F, t60738: F, t4425: F, t1630: F, t60730: F, t18436: F, t4409: F, t18444: F, t339: F, t4419: F, t790: F, t1246: F, t136: F, t1693: F, t19468: F, t19470: F, t5543: F) -> (F, F, F, F, F, F, F) {
    let t65551 = t60738 * t4466;
    let t65552 = 7.0 / 1152.0 * t65551;
    let t65561 = t60738 * t4425;
    let t65562 = 7.0 / 288.0 * t65561;
    let t65567 = t60730 * t1630;
    let t65570 = t18436 * t4409;
    let t65571 = 7.0 / 72.0 * t65570;
    let t65592 = t339 * t18444 * t790 * t4419;
    let t65593 = 7.0 / 576.0 * t65592;
    let t65595 = t1693 * t1246 * t136;
    let t65600 = t5543 * t19468 * t19470;
    (t65552, t65562, t65567, t65571, t65593, t65595, t65600)
}
