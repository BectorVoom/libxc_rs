//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 613/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk613<F: Float>(t240: F, t59: F, t2262: F, t2268: F, t2271: F, t2275: F, t2278: F, t39: F, t44: F, t51: F, t615: F, t618: F) -> (F, F, F) {
    let t2281 = t59 * t240;
    let t2282 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t2281;
    let t2283 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t2262 * t44 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t615 * t618 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t39 * t2268 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t2271 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t51 * t2275 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t2278 - t2282;
    (t2281, t2282, t2283)
}
