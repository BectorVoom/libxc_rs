//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1361/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1361<F: Float>(t5705: F, t2815: F, t41904: F, t47787: F, t59657: F, t68442: F, t76574: F, t76578: F, t76583: F, t76587: F, t76591: F, t76595: F, t76599: F) -> (F, F, F) {
    let t77041 = t5705 * t5705;
    let t77042 = t2815 * t77041;
    let t77058 = F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t47787 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t76574 - t76578 / F::cast_from(3.0_f64) - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t59657 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t76583 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t76587 - F::cast_from(8.0_f64) * t76591 + F::cast_from(8.0_f64) * t76595 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t76599 + t41904 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t68442;
    (t77041, t77042, t77058)
}
