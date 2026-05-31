//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1087/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1087<F: Float>(t1411: F, t1427: F, t1434: F, t3962: F, t3968: F, t3971: F, t3976: F, t3998: F, t4018: F, t609: F, t629: F, t642: F, t66: F, t80: F) -> F {
    let t4021 = -t3962 * t80 / F::cast_from(12.0_f64) - t3968 * t80 / F::cast_from(12.0_f64) - t3971 * t80 / F::cast_from(12.0_f64) - t1411 * t642 / F::cast_from(12.0_f64) - t3976 * t80 / F::cast_from(12.0_f64) + t3998 * t80 / F::cast_from(24.0_f64) + t1427 * t642 / F::cast_from(24.0_f64) - t609 * t1434 / F::cast_from(12.0_f64) + t629 * t1434 / F::cast_from(24.0_f64) + t66 * t4018 / F::cast_from(24.0_f64);
    t4021
}
