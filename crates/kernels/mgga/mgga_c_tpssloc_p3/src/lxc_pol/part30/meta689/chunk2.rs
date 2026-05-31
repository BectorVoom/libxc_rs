//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2197/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2197<F: Float>(t22574: F, t74060: F, t8643: F, t1388: F, t28830: F, t26162: F, t1983: F, t28238: F, t6999: F, t75214: F, t12461: F, t7752: F) -> (F, F, F, F, F) {
    let t97910 = F::cast_from(6.0_f64) * t22574 * t8643 * t74060;
    let t97911 = t28830 * t1388;
    let t97914 = F::cast_from(12.0_f64) * t22574 * t26162 * t97911;
    let t97916 = t1983 * t28238 * t6999;
    let t97919 = F::cast_from(3.0_f64) * t22574 * t8643 * t75214;
    let t97920 = t7752 * t12461;
    (t97910, t97914, t97916, t97919, t97920)
}
