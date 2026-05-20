//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1313/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1313<F: Float>(t2570: F, t792: F, t118: F, t2379: F, t794: F, t2553: F, t2576: F, t154: F, t845: F, t205: F, t59: F, t8705: F) -> (F, F, F, F, F) {
    let t9549 = t792 * t2570;
    let t9551 = t118 * t794 * t2379;
    let t9552 = t9549 * t9551;
    let t9555 = t118 * t794 * t2553;
    let t9556 = t2576 * t9555;
    let t9558 = t154 * t845;
    let t9559 = t205 * t9558;
    let t9569 = t59 * t8705;
    (t9552, t9556, t9558, t9559, t9569)
}
