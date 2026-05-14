//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 760/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk760<F: Float>(t3: F, t5465: F, t116: F, t4637: F, t117: F, t4674: F, t1668: F, t1670: F, t547: F, t548: F, t1976: F, t38: F) -> (F, F, F, F, F, F) {
    let t5466 = t3 * t5465;
    let t5470 = param_d * t5465;
    let t5474 = t116 * t4637;
    let t5477 = t117 * t4674;
    let t5480 = 6.0 * t1668 * t1670 + 6.0 * t547 * t5474 + 3.0 * t547 * t5477 + t5470 * t548;
    let t5483 = t1976 * t38;
    (t5466, t5470, t5474, t5477, t5480, t5483)
}
