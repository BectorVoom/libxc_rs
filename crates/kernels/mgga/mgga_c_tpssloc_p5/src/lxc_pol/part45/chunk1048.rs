//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1048/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1048<F: Float>(t22574: F, t31299: F, t32193: F, t22480: F, t7042: F, t23929: F, t8526: F, t1307: F, t26558: F, t31775: F, t22607: F, t8641: F) -> (F, F, F, F, F) {
    let t115942 = F::new(6.0) * t22574 * t32193 * t31299;
    let t115946 = F::new(2.0) * t7042 * t22480;
    let t115948 = F::new(4.0) * t8526 * t23929;
    let t115959 = F::new(12.0) * t22574 * t26558 * t31775 * t1307;
    let t115965 = t22607 * t8641;
    (t115942, t115946, t115948, t115959, t115965)
}
