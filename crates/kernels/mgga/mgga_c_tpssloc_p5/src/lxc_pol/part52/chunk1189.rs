//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1189/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1189<F: Float>(t1266: F, t1393: F, t1976: F, t2114: F, t2165: F, t31078: F, t31080: F, t31082: F, t31088: F, t31089: F, t31223: F, t31877: F, t31892: F, t510: F, t574: F, t6515: F, t6862: F, t7264: F, t8667: F, t8687: F) -> F {
    let t31895 = -t1266 * t8667 + t1393 * t8687 - t1976 * t7264 - t2114 * t6862 - t2165 * t6515 - t31877 * t510 + t31892 * t574 - F::new(2.0) * t31078 - F::new(2.0) * t31080 - F::new(2.0) * t31082 - t31088 + t31089 + t31223;
    t31895
}
