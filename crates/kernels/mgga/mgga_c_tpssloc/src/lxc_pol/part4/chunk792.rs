//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 792/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk792<F: Float>(t2559: F, t786: F, t789: F, t2566: F, t2578: F, t2570: F, t792: F, t154: F, t845: F, t205: F, t59: F, t8705: F, t207: F, t215: F, t782: F, t2690: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9541 = t2559 * t786;
    let t9542 = t9541 * t789;
    let t9546 = t2566 * t786;
    let t9547 = t9546 * t2578;
    let t9549 = t792 * t2570;
    let t9558 = t154 * t845;
    let t9559 = t205 * t9558;
    let t9569 = t59 * t8705;
    let t9572 = 0.28086419753086419752e-1 * t9569 * t207 * t215;
    let t9573 = t782 * t2570;
    let t9576 = t59 * t2690;
    (t9541, t9542, t9546, t9547, t9549, t9558, t9559, t9569, t9572, t9573, t9576)
}
