//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 911/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk911<F: Float>(t441: F, t9533: F, t461: F, t650: F, t1114: F, t242: F, t1111: F, t3065: F, t8507: F, t3124: F, t3090: F, t774: F) -> (F, F, F, F, F, F) {
    let t9535 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t441 * t9533;
    let t9540 = t650 * t461;
    let t9542 = t242 * t9540 * t1114;
    let t9543 = t1111 * t9542;
    let t9555 = t3065 * t8507;
    let t9556 = t3124 * t9555;
    let t9561 = t774 * t3090;
    (t9535, t9540, t9543, t9555, t9556, t9561)
}
