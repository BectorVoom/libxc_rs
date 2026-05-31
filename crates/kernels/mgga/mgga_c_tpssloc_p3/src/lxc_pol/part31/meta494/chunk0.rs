//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1686/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1686<F: Float>(t1774: F, t7056: F, t1266: F, t7801: F, t12725: F, t1442: F, t1459: F, t2036: F, t2040: F, t2075: F, t2314: F, t23938: F, t27188: F, t27215: F, t4026: F, t4034: F, t4073: F, t4077: F, t5107: F, t574: F, t652: F, t672: F, t7040: F, t7042: F, t7156: F, t7787: F, t7802: F) -> (F, F, F) {
    let t27219 = t1774 * t7056;
    let t27226 = t1266 * t7801;
    let t27238 = -t1266 * t7787 - F::cast_from(2.0_f64) * t12725 * t2040 - t1442 * t7156 - F::cast_from(2.0_f64) * t1459 * t23938 - t1774 * t7040 - t2036 * t5107 - t2075 * t4026 - F::cast_from(2.0_f64) * t2314 * t7802 - F::cast_from(2.0_f64) * t27188 * t672 + t27215 * t574 - F::cast_from(2.0_f64) * t27219 * t652 - F::cast_from(2.0_f64) * t27226 * t652 - F::cast_from(2.0_f64) * t4034 * t7802 - F::cast_from(2.0_f64) * t4073 * t7042 - F::cast_from(2.0_f64) * t4077 * t7042;
    (t27219, t27226, t27238)
}
