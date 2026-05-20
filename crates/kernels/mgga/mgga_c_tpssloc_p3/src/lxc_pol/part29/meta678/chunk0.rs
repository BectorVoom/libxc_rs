//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2270/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2270<F: Float>(t2363: F, t3941: F, t7467: F, t12724: F, t12728: F, t16503: F, t2165: F, t2167: F, t2364: F, t24552: F, t27858: F, t27863: F, t4028: F, t4072: F, t650: F, t652: F, t7408: F, t7989: F, t86673: F, t86676: F, t86679: F, t86682: F, t86684: F, t86688: F, t86693: F, t86698: F, t86700: F, t86702: F, t90020: F, t9348: F) -> (F, F) {
    let t91802 = F::new(27.0) * t3941 * t7467 * t2363;
    let t94223 = -F::new(4.0) * t4072 * t652 * t7408 - t12724 * t2165 - F::new(2.0) * t12728 * t2165 + t16503 * t2167 - F::new(2.0) * t2364 * t27863 - F::new(2.0) * t24552 * t4028 - F::new(2.0) * t27858 * t650 - F::new(2.0) * t7989 * t9348 + t86673 + t86676 + t86679 + t86682 - t86684 - t86688 + t86693 - t86698 - t86700 - t86702 + t90020;
    (t91802, t94223)
}
