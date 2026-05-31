//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2282/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2282<F: Float>(t17632: F, t17637: F, t17643: F, t17688: F, t17718: F, t17976: F, t17980: F, t23541: F, t25580: F, t4585: F, t4590: F, t6765: F, t82885: F, t83065: F, t88281: F) -> F {
    let t99535 = -t6765 * t17976 / F::cast_from(576.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t6765 * t17688 - t6765 * t17637 / F::cast_from(1152.0_f64) - t25580 * t4585 / F::cast_from(576.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t25580 * t4590 + t88281 + t82885 / F::cast_from(1296.0_f64) - t23541 * t17718 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t6765 * t17643 - t23541 * t17632 / F::cast_from(768.0_f64) + t83065 * t17980 / F::cast_from(1536.0_f64);
    t99535
}
