//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 437/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk437<F: Float>(t259: F, t479: F, t1578: F, t1561: F, t466: F, t1141: F, t1143: F, t220: F, t468: F, t1139: F, t1136: F, t473: F, t1153: F, t1402: F, t1507: F, t1521: F, t1547: F, t1549: F, t1553: F, t198: F, t330: F) -> (F, F, F, F, F, F) {
    let t480 = t259 < t479;
    let t1579 = param_beta * t1578;
    let t1581 = t466 * t1561;
    let t1586 = t1141 * t1143 * t1581 + t1578 * t220 * t468;
    let t1587 = t1139 * t1586;
    let t1589 = -t1136 * t1587 + t1579 * t473;
    let t1594 = piecewise3(t480, t1153 * t1589 * t198 * t330 - t1507 + t1521 + t1547 + t1549 - t1553, t1402);
    (t1579, t1581, t1586, t1587, t1589, t1594)
}
