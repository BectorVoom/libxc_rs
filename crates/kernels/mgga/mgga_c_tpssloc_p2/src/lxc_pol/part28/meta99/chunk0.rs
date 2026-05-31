//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 614/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk614<F: Float>(t2283: F, t33: F, t40: F, t632: F, t73: F, t52: F, t636: F, t76: F, t2244: F, t2250: F, t634: F, t638: F) -> (F, F, F, F, F, F) {
    let t2284 = t33 * t2283;
    let t2289 = t632 * t40;
    let t2291 = F::cast_from(1.0_f64) / t73 / t2289;
    let t2296 = t636 * t52;
    let t2298 = F::cast_from(1.0_f64) / t76 / t2296;
    let t2303 = F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t2291 * t2244 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t634 * t2250 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t2298 * t2244 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t638 * t2250;
    (t2284, t2289, t2291, t2296, t2298, t2303)
}
