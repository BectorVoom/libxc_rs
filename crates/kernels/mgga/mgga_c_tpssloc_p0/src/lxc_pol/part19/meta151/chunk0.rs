//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 761/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk761<F: Float>(t1351: F, t3792: F, t546: F, t68: F, t3787: F, t544: F, t1338: F, t641: F, t71: F, t154: F, t781: F, t202: F, t243: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5250 = t3792 * t1351;
    let t5278 = t546 * t68;
    let t5333 = t68 * t3787;
    let t5334 = t544 * t5333;
    let t5343 = t68 * t1338;
    let t5344 = t544 * t5343;
    let t6509 = t71 * t641;
    let t6546 = t781 * t154;
    let t6589 = F::new(1.0) / t243 / t202;
    (t5250, t5278, t5333, t5334, t5343, t5344, t6509, t6546, t6589)
}
