//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 791/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk791<F: Float>(t116: F, t4637: F, t117: F, t4674: F, t1668: F, t1670: F, t547: F, t5470: F, t548: F, t1976: F, t38: F, t1677: F) -> (F, F, F, F, F) {
    let t5474 = t116 * t4637;
    let t5477 = t117 * t4674;
    let t5480 = 6.0 * t1668 * t1670 + 6.0 * t547 * t5474 + 3.0 * t547 * t5477 + t5470 * t548;
    let t5483 = t1976 * t38;
    let t5486 = t38 * t1677;
    (t5474, t5477, t5480, t5483, t5486)
}
