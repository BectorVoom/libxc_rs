//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 187/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk187<F: Float>(t738: F, t172: F, t688: F, t690: F, t694: F, t699: F, t180: F) -> (F, F, F, F) {
    let t739 = 1.0 / t738;
    let t740 = t172 * t739;
    let t745 = -0.86308333333333333334e0 * t688 - 0.301925e0 * t690 - 0.5501625e-1 * t694 - 0.82785e-1 * t699;
    let t746 = 1.0 / t180;
    (t739, t740, t745, t746)
}
