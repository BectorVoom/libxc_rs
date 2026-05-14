//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 752/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk752<F: Float>(t5657: F, t858: F, t1528: F, t259: F, t4147: F, t4268: F, t5559: F, t5561: F, t5632: F, t5637: F, t855: F, t1530: F, t193: F, t202: F, t2378: F, t2423: F, t2426: F, t2486: F, t2518: F, t2530: F, t2537: F, t2665: F, t2752: F, t5527: F, t5544: F, t5596: F, t5599: F, t766: F, t870: F) -> (F, F, F, F) {
    let t5658 = t858 * t5657;
    let t5660 = -2.0 * t1528 * t4147 - 2.0 * t1528 * t4268 + t259 * t5559 + 2.0 * t259 * t5561 + t259 * t5632 + 2.0 * t5637 * t855 - t5658 * t855;
    let t5664 = t1530 * t1530;
    let t5668 = -t193 * t202 * t2752 * t5664 + t193 * t202 * t5660 * t870 + 6.0 * t193 * t2378 * t5527 + 3.0 * t193 * t5544 * t766 - t2423 - t2426 - t2486 + t2518 - t2530 - t2537 + t2665 - t5596 + t5599;
    (t5658, t5660, t5664, t5668)
}
