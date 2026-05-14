//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 372/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk372<F: Float>(t1208: F, t476: F, t478: F, t3036: F, t483: F, t3500: F, t475: F, t1210: F, t121: F, t1229: F, t374: F, t486: F, t677: F, t485: F, t221: F, t3426: F) -> (F, F, F, F, F, F, F) {
    let t3502 = 1.0 / t1208 / t476;
    let t3503 = t3502 * t478;
    let t3504 = t483 * t3036;
    let t3505 = t3503 * t3504;
    let t3506 = t3500 * t3505;
    let t3508 = t475 * t475;
    let t3514 = t1210 * t3504;
    let t3515 = t3500 * t3514;
    let t3521 = t121 * t1229;
    let t3540 = t374 * t677 * t486;
    let t3542 = t485 * t3540 / 13824.0;
    let t3545 = t221 * t3426;
    (t3502, t3506, t3508, t3515, t3521, t3542, t3545)
}
