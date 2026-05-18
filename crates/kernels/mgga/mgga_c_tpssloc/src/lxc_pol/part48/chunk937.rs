//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 937/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk937<F: Float>(t31124: F, t6883: F, t31101: F, t81159: F, t22635: F, t26331: F, t31099: F, t3734: F, t22716: F, t8455: F, t1985: F, t214: F, t225: F, t22870: F, t567: F) -> (F, F, F, F, F) {
    let t114253 = t6883 * t31124;
    let t114254 = F::new(0.76763589786250567036e-1) * t114253;
    let t114255 = t81159 * t31101;
    let t114256 = F::new(0.15352717957250113407e0) * t114255;
    let t114262 = F::new(0.9869604401089358619e-1) * t26331 * t22635 * t31099 * t3734;
    let t114264 = F::new(0.12793931631041761173e0) * t22716 * t8455;
    let t114270 = F::new(0.16449340668482264365e-1) * t1985 * t214 * t22870 * t225 * t567;
    (t114254, t114256, t114262, t114264, t114270)
}
