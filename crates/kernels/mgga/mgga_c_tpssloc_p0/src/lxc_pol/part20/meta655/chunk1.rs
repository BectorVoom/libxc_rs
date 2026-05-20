//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2423/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2423<F: Float>(t10623: F, t4498: F, t4493: F, t10629: F, t14259: F, t4471: F, t959: F, t14260: F, t2940: F, t13663: F, t13718: F, t49082: F, t49084: F, t49086: F, t49088: F, t49090: F, t49092: F, t49095: F, t49228: F, t49244: F, t49535: F, t49538: F, t49540: F, t49544: F, t49548: F) -> (F, F, F, F, F, F, F) {
    let t49550 = F::cast_from(0.51947577317044391277e2_f64) * t10623 * t4498;
    let t49552 = F::cast_from(0.17544670867903938621e1_f64) * t10623 * t4493;
    let t49556 = F::cast_from(0.30762056574649219973e4_f64) * t959 * t10629 * t4471 * t14259;
    let t49558 = F::cast_from(0.30762056574649219973e4_f64) * t2940 * t14260;
    let t49560 = F::cast_from(0.70178683471615754484e1_f64) * t2940 * t13663;
    let t49562 = F::cast_from(0.17544670867903938621e1_f64) * t2940 * t13718;
    let t49563 = -t49082 + t49084 - t49086 + t49088 - t49090 + t49092 - t49095 + t49535 + t49538 - t49540 - t49544 + t49548 - t49550 + t49228 - t49552 - t49556 - t49558 + t49560 - t49562 + t49244;
    (t49550, t49552, t49556, t49558, t49560, t49562, t49563)
}
