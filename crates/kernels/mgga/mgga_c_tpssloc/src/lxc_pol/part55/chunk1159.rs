//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1159/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1159<F: Float>(t31109: F, t6883: F, t31124: F, t31101: F, t81159: F, t22716: F, t8455: F, t22704: F, t31091: F, t81326: F, t2006: F, t213: F, t225: F) -> (F, F, F, F, F, F) {
    let t114242 = t6883 * t31109;
    let t114253 = t6883 * t31124;
    let t114255 = t81159 * t31101;
    let t114264 = F::new(0.12793931631041761173e0) * t22716 * t8455;
    let t114278 = t22704 * t81326 * t31091;
    let t114285 = t213 * t2006 * t225;
    (t114242, t114253, t114255, t114264, t114278, t114285)
}
