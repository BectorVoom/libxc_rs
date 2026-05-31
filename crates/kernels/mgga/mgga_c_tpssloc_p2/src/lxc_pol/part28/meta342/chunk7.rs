//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1294/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1294<F: Float>(t12545: F, t12550: F, t12557: F, t12725: F, t12734: F, t12816: F, t12823: F, t1442: F, t1459: F, t1849: F, t2314: F, t2323: F, t2364: F, t3652: F, t3660: F, t4028: F, t4034: F, t4037: F, t4073: F, t574: F, t652: F, t672: F, t9348: F) -> F {
    let t12832 = -F::cast_from(4.0_f64) * t12545 * t652 - F::cast_from(4.0_f64) * t12550 * t652 - F::cast_from(2.0_f64) * t12557 * t652 - F::cast_from(4.0_f64) * t12725 * t672 - F::cast_from(4.0_f64) * t12734 * t1459 + t12816 * t574 - F::cast_from(2.0_f64) * t12823 * t1459 - t1442 * t3652 - F::cast_from(2.0_f64) * t1459 * t9348 + t1849 * t3660 - F::cast_from(4.0_f64) * t2314 * t4073 - F::cast_from(4.0_f64) * t2323 * t4028 - F::cast_from(2.0_f64) * t2364 * t4028 - F::cast_from(4.0_f64) * t4034 * t4037 - F::cast_from(4.0_f64) * t4034 * t4073;
    t12832
}
