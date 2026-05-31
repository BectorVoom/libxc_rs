//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 240/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk240<F: Float>(t626: F, t39: F, t44: F, t51: F, t615: F, t618: F, t621: F) -> (F, F) {
    let t627 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t626;
    let t628 = -F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t615 * t44 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t618 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t621 + t627;
    (t627, t628)
}
