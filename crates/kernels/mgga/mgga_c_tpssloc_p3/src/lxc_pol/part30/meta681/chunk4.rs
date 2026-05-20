//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2142/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2142<F: Float>(t22881: F, t6347: F, t6637: F, t6888: F, t19631: F, t6968: F, t22705: F, t28130: F, t81228: F, t19748: F, t1992: F, t22897: F) -> (F, F, F, F) {
    let t97036 = t6888 * t6637 * t22881 * t6347;
    let t97040 = t6888 * t6637 * t6968 * t19631;
    let t97043 = t81228 * t22705 * t28130;
    let t97046 = t1992 * t22897 * t19748;
    (t97036, t97040, t97043, t97046)
}
