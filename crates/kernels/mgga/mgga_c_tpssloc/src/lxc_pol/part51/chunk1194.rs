//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1194/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1194<F: Float>(t1266: F, t2040: F, t22461: F, t26103: F, t31055: F, t31057: F, t31060: F, t31700: F, t31733: F, t31734: F, t31737: F, t31746: F, t31749: F, t31750: F, t510: F, t6517: F, t652: F, t7050: F, t8519: F) -> F {
    let t31751 = -t1266 * t8519 - F::new(2.0) * t2040 * t22461 - F::new(2.0) * t2040 * t26103 - t31700 * t510 - F::new(2.0) * t31734 * t652 - F::new(2.0) * t6517 * t7050 - t31055 - t31057 - t31060 - t31733 + t31737 - t31746 - t31749 - t31750;
    t31751
}
