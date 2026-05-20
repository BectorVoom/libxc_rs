//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1918/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1918<F: Float>(t22705: F, t26422: F, t81228: F, t16040: F, t22633: F, t3807: F, t6976: F, t1992: F, t54854: F, t550: F, t26331: F, t26421: F, t26446: F, t3719: F) -> (F, F, F, F) {
    let t90844 = t81228 * t22705 * t26422;
    let t90848 = t22633 * t6976 * t16040 * t3807;
    let t90852 = t1992 * t6976 * t54854 * t550;
    let t90856 = t26331 * t26446 * t26421 * t3719;
    (t90844, t90848, t90852, t90856)
}
