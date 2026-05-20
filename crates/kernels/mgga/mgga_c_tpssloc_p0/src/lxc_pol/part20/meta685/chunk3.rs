//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2598/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2598<F: Float>(t15495: F, t3572: F, t1227: F, t1653: F, t248: F, t45293: F, t15591: F, t15643: F, t3490: F, t1089: F, t3507: F, t607: F) -> (F, F, F, F, F) {
    let t52674 = t15495 * t3572;
    let t52680 = t1227 * t248 * t45293 * t1653;
    let t52682 = t15591 * t3572;
    let t52684 = t3490 * t15643;
    let t52687 = t3507 * t1089 * t607;
    (t52674, t52680, t52682, t52684, t52687)
}
