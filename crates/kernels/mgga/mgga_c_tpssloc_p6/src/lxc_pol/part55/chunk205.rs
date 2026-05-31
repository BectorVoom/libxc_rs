//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 205/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk205<F: Float>(t167: F, t730: F, t177: F, t172: F, t688: F, t690: F, t694: F, t699: F, t180: F) -> (F, F, F, F, F, F, F) {
    let t731 = F::cast_from(1.0_f64) / t167;
    let t732 = t730 * t731;
    let t738 = t177 * t177;
    let t739 = F::cast_from(1.0_f64) / t738;
    let t740 = t172 * t739;
    let t745 = -F::cast_from(0.86308333333333333334e0_f64) * t688 - F::cast_from(0.301925e0_f64) * t690 - F::cast_from(0.5501625e-1_f64) * t694 - F::cast_from(0.82785e-1_f64) * t699;
    let t746 = F::cast_from(1.0_f64) / t180;
    (t731, t732, t738, t739, t740, t745, t746)
}
