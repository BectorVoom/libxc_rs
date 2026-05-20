//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1376/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1376<F: Float>(t42212: F, t59688: F, t59694: F, t76574: F, t76578: F, t76583: F, t76591: F, t76599: F, t76614: F, t76622: F, t76893: F, t76896: F, t76909: F, t76915: F) -> F {
    let t77287 = -F::new(0.125034e1) * t76893 + F::cast_from(0.55570666666666666666e0_f64) * t76896 + F::new(0.250068e1) * t76909 + F::new(0.62517e0) * t76915 - F::cast_from(0.15302962962962962963e1_f64) * t76574 - F::new(0.516475e0) * t76578 + F::cast_from(0.68863333333333333334e1_f64) * t76583 - F::new(0.123954e2) * t76591 - F::new(0.103295e1) * t76599 + F::new(0.123954e2) * t76614 + F::new(0.309885e1) * t76622 + F::cast_from(0.27545333333333333333e1_f64) * t59688 - F::cast_from(0.13772666666666666666e1_f64) * t59694 + t42212;
    t77287
}
