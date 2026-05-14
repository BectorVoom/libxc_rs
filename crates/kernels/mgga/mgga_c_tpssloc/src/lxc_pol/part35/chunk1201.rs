//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1201/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1201<F: Float>(t24574: F, t29702: F, t6260: F, t7327: F, t24660: F, t6252: F, t27736: F, t7999: F, t24826: F, t29716: F, t8070: F, t94490: F, t8074: F, t94909: F, t29745: F, t29705: F) -> (F, F, F, F, F, F, F, F, F) {
    let t103744 = t24574 * t29702;
    let t103767 = t7327 * t6260;
    let t103774 = t24660 * t6252;
    let t103799 = t7999 * t27736;
    let t103810 = t24826 * t29716;
    let t103830 = t94490 * t8070;
    let t103867 = t94909 * t8074;
    let t103877 = t24826 * t29745;
    let t103879 = t24574 * t29705;
    (t103744, t103767, t103774, t103799, t103810, t103830, t103867, t103877, t103879)
}
