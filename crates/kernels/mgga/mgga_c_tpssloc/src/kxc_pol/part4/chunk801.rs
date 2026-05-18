//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 801/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk801<F: Float>(t1055: F, t5943: F, t1052: F, t1635: F, t388: F, t4557: F, t4660: F, t5849: F, t5851: F, t5915: F, t5920: F, t1637: F) -> (F, F, F) {
    let t5944 = t1055 * t5943;
    let t5946 = F::new(2.0) * t1052 * t5920 - t1052 * t5944 - F::new(2.0) * t1635 * t4557 - F::new(2.0) * t1635 * t4660 + t388 * t5849 + F::new(2.0) * t388 * t5851 + t388 * t5915;
    let t5950 = t1637 * t1637;
    (t5944, t5946, t5950)
}
