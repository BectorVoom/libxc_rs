//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2057/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2057<F: Float>(t27495: F, t85821: F, t15702: F, t7329: F, t1011: F, t3493: F, t225: F, t24698: F, t1193: F, t24811: F, t24817: F, t24823: F) -> (F, F, F, F, F, F, F) {
    let t85822 = t85821 * t27495;
    let t85824 = t85822 * t7329 * t15702;
    let t85827 = t3493 * t1011;
    let t85832 = t24698 * t225;
    let t85853 = t24811 * t1193;
    let t85854 = t85853 * t24817;
    let t85883 = t85853 * t24823;
    (t85822, t85824, t85827, t85832, t85853, t85854, t85883)
}
