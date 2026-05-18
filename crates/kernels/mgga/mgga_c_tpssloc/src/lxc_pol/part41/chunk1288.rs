//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1288/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1288<F: Float>(t5464: F, t666: F, t1453: F, t4067: F, t5488: F, t19534: F, t89: F, t1395: F, t5456: F, t626: F, t9365: F, t45435: F, t64: F) -> (F, F, F, F, F, F, F) {
    let t96715 = t5464 * t666;
    let t96718 = t1453 * t4067;
    let t96723 = t5488 * t666;
    let t97933 = t89 * t19534;
    let t100930 = t1395 * t5456;
    let t110075 = t626 * t9365;
    let t110082 = t64 * t45435;
    (t96715, t96718, t96723, t97933, t100930, t110075, t110082)
}
