//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1941/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1941<F: Float>(t22765: F, t5289: F, t22764: F, t5234: F, t1354: F, t26298: F, t80958: F, t1307: F, t1339: F, t22827: F, t5287: F, t54068: F, t550: F) -> (F, F, F, F, F) {
    let t91283 = t22765 * t5289;
    let t91285 = t5234 * t22764;
    let t91286 = t91285 * t1354;
    let t91290 = t80958 * t26298;
    let t91294 = t22827 * t1339 * t5287 * t1307;
    let t91298 = t22827 * t1339 * t54068 * t550;
    (t91283, t91286, t91290, t91294, t91298)
}
