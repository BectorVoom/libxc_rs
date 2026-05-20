//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 705/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk705<F: Float>(t1203: F, t1222: F, t221: F, t3426: F, t456: F, t1197: F, t135: F, t1174: F, t1176: F, t3247: F, t3242: F, t3439: F) -> (F, F, F, F, F, F, F) {
    let t3543 = t1203 * t1222;
    let t3545 = t221 * t3426;
    let t3547 = t456 * t3545 / F::new(432.0);
    let t3548 = t135 * t1197;
    let t3549 = t1174 * t3548;
    let t3555 = t1176 * t3247;
    let t3560 = t3439 * t3242;
    (t3543, t3545, t3547, t3548, t3549, t3555, t3560)
}
