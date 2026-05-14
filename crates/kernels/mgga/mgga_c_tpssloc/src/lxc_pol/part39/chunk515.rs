//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 515/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk515<F: Float>(t2281: F, t2262: F, t2268: F, t2271: F, t2275: F, t2278: F, t39: F, t44: F, t51: F, t615: F, t618: F, t33: F, t40: F, t632: F, t73: F, t52: F, t636: F) -> (F, F, F, F, F, F) {
    let t2282 = 88.0 / 9.0 * t2281;
    let t2283 = 88.0 / 9.0 * t2262 * t44 - 40.0 / 9.0 * t615 * t618 + 5.0 / 18.0 * t39 * t2268 + 5.0 / 6.0 * t39 * t2271 + 5.0 / 18.0 * t51 * t2275 - 5.0 / 6.0 * t51 * t2278 - t2282;
    let t2284 = t33 * t2283;
    let t2289 = t632 * t40;
    let t2291 = 1.0 / t73 / t2289;
    let t2296 = t636 * t52;
    (t2282, t2283, t2284, t2289, t2291, t2296)
}
