//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 890/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk890<F: Float>(t31759: F, t6876: F, t22573: F, t8606: F, t22575: F, t31526: F, t22574: F, t31299: F, t32193: F, t22480: F, t7042: F, t23929: F, t8526: F, t1307: F, t26558: F, t31775: F) -> (F, F, F, F, F, F, F) {
    let t115924 = 6.0 * t6876 * t31759;
    let t115925 = t8606 * t22573;
    let t115927 = 6.0 * t115925 * t22575;
    let t115929 = 2.0 * t6876 * t31526;
    let t115942 = 6.0 * t22574 * t32193 * t31299;
    let t115946 = 2.0 * t7042 * t22480;
    let t115948 = 4.0 * t8526 * t23929;
    let t115959 = 12.0 * t22574 * t26558 * t31775 * t1307;
    (t115924, t115927, t115929, t115942, t115946, t115948, t115959)
}
