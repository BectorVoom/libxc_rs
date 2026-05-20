//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2281/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2281<F: Float>(t25577: F, t4630: F, t25580: F, t4571: F, t17906: F, t6765: F, t17884: F, t17655: F, t23541: F, t1618: F, t17972: F, t23433: F, t23529: F, t4575: F, t5869: F, t5900: F, t82875: F, t88251: F, t88513: F, t88591: F) -> F {
    let t99495 = t25577 * t4630;
    let t99497 = t25580 * t4571;
    let t99501 = t6765 * t17906;
    let t99507 = t6765 * t17884;
    let t99509 = t23541 * t17655;
    let t99514 = -t88251 + t88513 * t4575 / F::new(1152.0) + t99495 / F::new(1152.0) + t99497 / F::new(1728.0) + t23529 * t5900 / F::new(216.0) - t99501 / F::new(1728.0) - t88591 * t1618 / F::new(144.0) + t23433 * t5869 / F::new(1536.0) + F::new(5.0) / F::new(10368.0) * t99507 - t99509 / F::new(2304.0) - t82875 / F::new(10368.0) + t6765 * t17972 / F::new(384.0);
    t99514
}
