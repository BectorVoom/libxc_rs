//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 981/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk981<F: Float>(t1998: F, t59: F, t6347: F, t6926: F, t22845: F, t6330: F, t22827: F, t28100: F, t6943: F, t6415: F, t6936: F, t6378: F, t8465: F, t8467: F) -> (F, F, F, F, F) {
    let t127263 = t6926 * t1998 * t59 * t6347;
    let t127267 = t22845 * t1998 * t59 * t6330;
    let t127270 = t22827 * t6943 * t28100;
    let t127273 = t6936 * t6943 * t6415;
    let t127278 = t6378 * t8465 * t8467;
    (t127263, t127267, t127270, t127273, t127278)
}
