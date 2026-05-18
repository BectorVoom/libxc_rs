//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 719/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk719<F: Float>(t252: F, t776: F, t829: F, t6646: F, t22986: F, t6624: F, t814: F, t2627: F, t6604: F, t2631: F, t2632: F, t1888: F) -> (F, F, F, F, F) {
    let t22987 = t252 * t776;
    let t22988 = t22987 * t829;
    let t22989 = t6646 * t22988;
    let t22990 = t22986 * t22989;
    let t22992 = t814 * t6624;
    let t22993 = t22992 * t829;
    let t22996 = t6604 * t2627;
    let t22997 = t252 * t2631;
    let t22998 = t22997 * t2632;
    let t22999 = t22996 * t22998;
    let t23000 = t1888 * t22999;
    (t22990, t22993, t22996, t22997, t23000)
}
