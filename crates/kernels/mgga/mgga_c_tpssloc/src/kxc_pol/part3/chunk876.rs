//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 876/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk876<F: Float>(t2566: F, t786: F, t2578: F, t2570: F, t792: F, t118: F, t2379: F, t794: F, t2553: F, t2576: F, t154: F, t845: F) -> (F, F, F, F, F) {
    let t9546 = t2566 * t786;
    let t9547 = t9546 * t2578;
    let t9549 = t792 * t2570;
    let t9551 = t118 * t794 * t2379;
    let t9552 = t9549 * t9551;
    let t9555 = t118 * t794 * t2553;
    let t9556 = t2576 * t9555;
    let t9558 = t154 * t845;
    (t9546, t9547, t9552, t9556, t9558)
}
