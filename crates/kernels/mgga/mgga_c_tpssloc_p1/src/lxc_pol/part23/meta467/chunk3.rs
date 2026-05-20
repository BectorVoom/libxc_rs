//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1371/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1371<F: Float>(t41959: F, t59688: F, t59694: F, t76574: F, t76578: F, t76583: F, t76591: F, t76599: F, t76614: F, t76622: F, t76893: F, t76896: F, t76909: F, t76915: F) -> F {
    let t77204 = -F::new(0.99342e0) * t76893 + F::new(0.44152e0) * t76896 + F::new(0.198684e1) * t76909 + F::new(0.49671e0) * t76915 - F::cast_from(0.89459259259259259259e0_f64) * t76574 - F::new(0.301925e0) * t76578 + F::cast_from(0.40256666666666666666e1_f64) * t76583 - F::new(0.72462e1) * t76591 - F::cast_from(0.60384999999999999999e0_f64) * t76599 + F::new(0.72462e1) * t76614 + F::new(0.181155e1) * t76622 + F::cast_from(0.16102666666666666667e1_f64) * t59688 - F::cast_from(0.80513333333333333336e0_f64) * t59694 + t41959;
    t77204
}
