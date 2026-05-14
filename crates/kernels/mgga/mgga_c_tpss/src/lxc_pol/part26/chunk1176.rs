//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1176/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1176<F: Float>(t19596: F, t485: F, t626: F, t5757: F, t6274: F, t1760: F, t1270: F, t4397: F, t5708: F, t5755: F, t6243: F, t1206: F, t1659: F, t7029: F, t18547: F, t18289: F, t6245: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19597 = t485 * t19596;
    let t19599 = 2.0 * t626 * t19597;
    let t19602 = t6274 * t5757;
    let t19603 = t1760 * t19602;
    let t19604 = t1270 * t4397;
    let t19605 = t5708 * t19604;
    let t19607 = 3.0 * t1760 * t19605;
    let t19608 = t6243 * t5755;
    let t19609 = t1659 * t1206;
    let t19610 = t7029 * t19609;
    let t19612 = 3.0 * t18547 * t19610;
    let t19614 = t18289 * t6245;
    (t19597, t19599, t19602, t19603, t19604, t19605, t19607, t19608, t19609, t19610, t19612, t19614)
}
