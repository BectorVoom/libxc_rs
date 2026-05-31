//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 778/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk778<F: Float>(t5657: F, t858: F, t1528: F, t259: F, t4147: F, t4268: F, t5559: F, t5561: F, t5632: F, t5637: F, t855: F, t1530: F) -> (F, F, F) {
    let t5658 = t858 * t5657;
    let t5660 = -F::cast_from(2.0_f64) * t1528 * t4147 - F::cast_from(2.0_f64) * t1528 * t4268 + t259 * t5559 + F::cast_from(2.0_f64) * t259 * t5561 + t259 * t5632 + F::cast_from(2.0_f64) * t5637 * t855 - t5658 * t855;
    let t5664 = t1530 * t1530;
    (t5658, t5660, t5664)
}
