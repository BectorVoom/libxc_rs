//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 614/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk614<F: Float>(t598: F, t6589: F, t213: F, t1894: F, t236: F, t776: F, t2229: F, t61: F) -> (F, F, F, F, F) {
    let t6590 = t598 * t6589;
    let t6591 = t6590 * t213;
    let t6593 = t1894 * t236 * t776;
    let t6594 = t6591 * t6593;
    let t6597 = 1.0 / t61 / t2229;
    (t6590, t6591, t6593, t6594, t6597)
}
