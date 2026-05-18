//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1353/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1353<F: Float>(t1011: F, t3493: F, t225: F, t24698: F, t11720: F, t2144: F, t1193: F, t24811: F, t24817: F, t24660: F, t7319: F, t24667: F) -> (F, F, F, F, F, F, F) {
    let t85827 = t3493 * t1011;
    let t85832 = t24698 * t225;
    let t85836 = t2144 * t11720;
    let t85853 = t24811 * t1193;
    let t85854 = t85853 * t24817;
    let t85859 = t7319 * t24660;
    let t85863 = t7319 * t24667;
    (t85827, t85832, t85836, t85853, t85854, t85859, t85863)
}
