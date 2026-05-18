//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1271/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1271<F: Float>(t19815: F, t6951: F, t22783: F, t6427: F, t236: F, t6387: F, t22705: F, t22852: F, t550: F, t28100: F, t80853: F, t80855: F) -> (F, F, F, F, F) {
    let t97265 = t19815 * t6951;
    let t97283 = t22783 * t6427;
    let t97312 = t236 * t6387;
    let t97315 = t22852 * t22705 * t97312 * t550;
    let t97347 = t80853 * t80855 * t28100;
    (t97265, t97283, t97312, t97315, t97347)
}
