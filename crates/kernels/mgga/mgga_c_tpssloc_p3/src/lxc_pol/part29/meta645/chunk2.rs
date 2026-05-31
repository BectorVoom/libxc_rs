//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2130/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2130<F: Float>(t23097: F, t2628: F, t2632: F, t47012: F, t23033: F, t25155: F, t6546: F, t13191: F, t221: F, t25154: F, t13196: F, t13171: F, t6605: F, t815: F) -> (F, F, F, F, F) {
    let t87458 = t23097 * t2628 * t47012 * t2632;
    let t87463 = t6546 * t23033 * t25155;
    let t87464 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t87463;
    let t87466 = t25154 * t221 * t13191;
    let t87469 = t25154 * t221 * t13196;
    let t87472 = t6605 * t815 * t13171;
    (t87458, t87464, t87466, t87469, t87472)
}
