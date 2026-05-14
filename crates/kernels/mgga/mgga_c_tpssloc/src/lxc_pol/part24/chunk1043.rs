//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1043/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1043<F: Float>(t16312: F, t550: F, t1339: F, t22827: F, t242: F, t6943: F, t1336: F) -> (F, F, F, F, F) {
    let t22828 = t16312 * t550;
    let t22829 = t1339 * t22828;
    let t22830 = t22827 * t22829;
    let t22832 = t6943 * t242;
    let t22833 = t1336 * t22832;
    (t22828, t22829, t22830, t22832, t22833)
}
