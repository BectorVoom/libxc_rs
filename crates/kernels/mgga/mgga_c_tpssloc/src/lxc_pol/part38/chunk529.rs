//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 529/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk529<F: Float>(t2281: F, t2262: F, t2268: F, t2271: F, t2275: F, t2278: F, t39: F, t44: F, t51: F, t615: F, t618: F, t33: F) -> (F, F, F) {
    let t2282 = F::new(88.0) / F::new(9.0) * t2281;
    let t2283 = F::new(88.0) / F::new(9.0) * t2262 * t44 - F::new(40.0) / F::new(9.0) * t615 * t618 + F::new(5.0) / F::new(18.0) * t39 * t2268 + F::new(5.0) / F::new(6.0) * t39 * t2271 + F::new(5.0) / F::new(18.0) * t51 * t2275 - F::new(5.0) / F::new(6.0) * t51 * t2278 - t2282;
    let t2284 = t33 * t2283;
    (t2282, t2283, t2284)
}
