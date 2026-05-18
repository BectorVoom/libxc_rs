//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 985/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk985<F: Float>(t22642: F, t22690: F, t31618: F, t115384: F, t1992: F, t22897: F, t3792: F, t22751: F, t31620: F, t552: F, t7191: F, t1307: F, t6637: F, t6888: F) -> (F, F, F, F) {
    let t115390 = t22642 * t22690 * t31618;
    let t115391 = F::new(0.82246703342411321824e-2) * t115390;
    let t115395 = t1992 * t22897 * t115384 * t3792;
    let t115397 = t22751 * t31620;
    let t115399 = t552 * t7191;
    let t115402 = t6888 * t6637 * t115399 * t1307;
    (t115391, t115395, t115397, t115402)
}
