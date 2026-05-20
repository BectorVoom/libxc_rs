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
    let t77058 = F::new(112.0) / F::new(81.0) * t47787 - F::new(80.0) / F::new(81.0) * t76574 - t76578 / F::new(3.0) - F::new(16.0) / F::new(27.0) * t59657 + F::new(40.0) / F::new(9.0) * t76583 - F::new(20.0) / F::new(9.0) * t76587 - F::new(8.0) * t76591 + F::new(8.0) * t76595 - F::new(2.0) / F::new(3.0) * t76599 + t41904 + F::new(8.0) / F::new(3.0) * t68442;
    (t77041, t77042, t77058)
}
