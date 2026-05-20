//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1238/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1238<F: Float>(t13360: F, t5628: F, t67441: F, t842: F, t5611: F, t9975: F, t21064: F, t225: F, t262: F, t5527: F, t21152: F, t690: F) -> (F, F, F, F, F, F) {
    let t68201 = t13360 * t5628;
    let t68203 = t67441 * t842;
    let t68246 = t9975 * t5611;
    let t68322 = t21064 * t225;
    let t68371 = t5527 * t262;
    let t68442 = t690 * t21152;
    (t68201, t68203, t68246, t68322, t68371, t68442)
}
