//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 209/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk209<F: Float>(t626: F, t39: F, t44: F, t51: F, t615: F, t618: F, t621: F, t33: F, t40: F) -> (F, F, F, F) {
    let t627 = F::new(8.0) / F::new(3.0) * t626;
    let t628 = -F::new(8.0) / F::new(3.0) * t615 * t44 + F::new(5.0) / F::new(6.0) * t39 * t618 - F::new(5.0) / F::new(6.0) * t51 * t621 + t627;
    let t629 = t33 * t628;
    let t632 = t40 * t40;
    (t627, t628, t629, t632)
}
