//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2568/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2568<F: Float>(t63911: F, t71144: F, t71400: F, t71403: F, t71406: F, t71408: F, t71411: F, t71414: F, t71417: F, t71420: F, t71423: F, t71426: F) -> F {
    let t71929 = -F::new(0.103295e1) * t71144 - F::cast_from(0.10805407407407407407e0_f64) * t71400 + F::cast_from(0.34731666666666666667e0_f64) * t63911 + F::new(0.3529725e1) * t71403 + F::new(0.104195e0) * t71406 - F::cast_from(0.69463333333333333333e-1_f64) * t71408 + F::cast_from(0.13892666666666666667e0_f64) * t71411 + F::cast_from(0.55570666666666666666e0_f64) * t71414 - F::new(0.62517e0) * t71417 - F::new(0.125034e1) * t71420 + F::new(0.187551e1) * t71423 + F::new(0.250068e1) * t71426;
    t71929
}
