//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1068/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1068<F: Float>(t2588: F, t40341: F, t207: F, t215: F, t39933: F, t40344: F, t795: F, t116: F, t786: F, t9534: F, t39568: F, t761: F, t39382: F, t39302: F, t6589: F, t68: F) -> (F, F, F, F, F, F, F, F) {
    let t41200 = 0.99537037037037037035e-1 * t40341 * t2588;
    let t41209 = 0.14979423868312757201e0 * t39933 * t207 * t215;
    let t41212 = 0.11265432098765432099e0 * t40344 * t207 * t795;
    let t41214 = t9534 * t786 * t116;
    let t41254 = 0.14035736694323150897e2 * t761 * t39568;
    let t41258 = 0.91082604192152556044e5 * t761 * t39382;
    let t41262 = 0.5848223622634646207e0 * t761 * t39302;
    let t41315 = t68 * t6589;
    (t41200, t41209, t41212, t41214, t41254, t41258, t41262, t41315)
}
