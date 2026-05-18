//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 972/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk972<F: Float>(t1985: F, t8458: F, t97511: F, t120550: F, t120568: F, t120576: F, t120446: F, t120458: F, t1998: F, t214: F, t28107: F, t120470: F) -> (F, F, F, F, F, F, F, F) {
    let t127349 = F::new(0.16449340668482264365e-1) * t1985 * t97511 * t8458;
    let t127350 = F::new(0.16449340668482264365e-1) * t120550;
    let t127354 = F::new(0.16449340668482264365e-1) * t120568;
    let t127355 = F::new(0.16449340668482264365e-1) * t120576;
    let t127356 = F::new(0.76763589786250567036e-1) * t120446;
    let t127357 = F::new(0.16449340668482264365e-1) * t120458;
    let t127361 = F::new(0.16449340668482264365e-1) * t1985 * t214 * t1998 * t28107;
    let t127362 = F::new(0.15352717957250113407e0) * t120470;
    (t127349, t127350, t127354, t127355, t127356, t127357, t127361, t127362)
}
