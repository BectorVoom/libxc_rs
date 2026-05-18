//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 232/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk232<F: Float>(t172: F, t739: F, t688: F, t690: F, t694: F, t699: F) -> (F, F) {
    let t740 = t172 * t739;
    let t745 = -F::new(0.86308333333333333334e0) * t688 - F::new(0.301925e0) * t690 - F::new(0.5501625e-1) * t694 - F::new(0.82785e-1) * t699;
    (t740, t745)
}
