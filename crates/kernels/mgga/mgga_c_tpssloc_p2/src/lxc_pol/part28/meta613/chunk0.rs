//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1927/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1927<F: Float>(t26421: F, t26446: F, t3734: F, t90591: F, t22751: F, t26389: F, t1992: F, t22897: F, t3792: F, t90870: F, t26467: F, t6914: F) -> (F, F, F, F) {
    let t91052 = t90591 * t26446 * t26421 * t3734;
    let t91064 = t22751 * t26389;
    let t91074 = t1992 * t22897 * t90870 * t3792;
    let t91076 = t6914 * t26467;
    (t91052, t91064, t91074, t91076)
}
