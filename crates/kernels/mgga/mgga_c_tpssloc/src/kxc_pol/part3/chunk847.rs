//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 847/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk847<F: Float>(t119: F, t5187: F, t210: F, t225: F, t5210: F, t554: F, t1814: F, t68: F) -> (F, F, F, F, F) {
    let t5226 = t119 * t5187;
    let t5227 = t210 * t5226;
    let t5230 = t5210 * t225;
    let t5231 = t5230 * t554;
    let t5234 = t1814 * t68;
    (t5226, t5227, t5230, t5231, t5234)
}
