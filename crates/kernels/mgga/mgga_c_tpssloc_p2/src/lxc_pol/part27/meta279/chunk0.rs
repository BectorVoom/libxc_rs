//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1321/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1321<F: Float>(t116: F, t131: F, t9537: F, t207: F, t9534: F, t2559: F, t786: F, t789: F, t2563: F, t2582: F, t2566: F, t2578: F) -> (F, F, F, F, F, F, F) {
    let t9538 = t116 * t131 * t9537;
    let t9540 = F::cast_from(0.13888888888888888889e-3_f64) * t9534 * t207 * t9538;
    let t9541 = t2559 * t786;
    let t9542 = t9541 * t789;
    let t9544 = t2563 * t2582;
    let t9546 = t2566 * t786;
    let t9547 = t9546 * t2578;
    (t9538, t9540, t9541, t9542, t9544, t9546, t9547)
}
