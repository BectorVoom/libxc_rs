//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2550/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2550<F: Float>(t63911: F, t71144: F, t71400: F, t71403: F, t71406: F, t71408: F, t71411: F, t71414: F, t71417: F, t71420: F, t71423: F, t71426: F) -> F {
    let t71585 = -F::cast_from(0.59793333333333333333e0_f64) * t71144 - F::cast_from(0.85199506172839506175e-1_f64) * t71400 + F::cast_from(0.27385555555555555555e0_f64) * t63911 + F::new(0.1898925e1) * t71403 + F::cast_from(0.82156666666666666667e-1_f64) * t71406 - F::cast_from(0.54771111111111111111e-1_f64) * t71408 + F::cast_from(0.10954222222222222222e0_f64) * t71411 + F::cast_from(0.43816888888888888889e0_f64) * t71414 - F::cast_from(0.49293999999999999999e0_f64) * t71417 - F::cast_from(0.98587999999999999998e0_f64) * t71420 + F::new(0.147882e1) * t71423 + F::new(0.197176e1) * t71426;
    t71585
}
