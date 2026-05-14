//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 736/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk736<F: Float>(t2240: F, t8301: F, t1862: F, t131: F, t68: F, t69: F, t79: F) -> (F, F, F, F, F) {
    let t8302 = t2240 * t8301;
    let t8303 = t1862 * t1862;
    let t8304 = t8303 * t131;
    let t8306 = 1.0 / t69 / t68;
    let t8307 = t79 * t79;
    (t8302, t8303, t8304, t8306, t8307)
}
