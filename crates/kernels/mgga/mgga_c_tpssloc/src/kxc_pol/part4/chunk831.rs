//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 831/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk831<F: Float>(t5962: F, t6286: F, t1268: F, t1458: F, t4028: F, t5450: F, t5456: F, t5493: F, t88: F, t5155: F, t5158: F, t1799: F, t5122: F) -> (F, F, F, F, F) {
    let t6287 = t5962 + t6286;
    let t6295 = F::new(2.0) * t1268 * t5493 + F::new(4.0) * t1458 * t4028 + F::new(2.0) * t5456 * t88 + t5450;
    let t6299 = F::new(0.11696447245269292414e1) * t5155;
    let t6300 = F::new(0.36622894612013090108e-3) * t5158;
    let t6301 = t5122 * t1799;
    (t6287, t6295, t6299, t6300, t6301)
}
