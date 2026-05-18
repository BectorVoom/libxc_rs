//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1234/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1234<F: Float>(t119867: F, t119869: F, t119871: F, t119874: F, t119875: F, t119877: F, t119996: F, t120002: F, t120003: F, t120005: F, t120008: F, t1266: F, t1442: F, t22461: F, t26103: F, t30989: F, t32679: F, t33124: F, t4026: F, t510: F, t7472: F, t8329: F, t8439: F) -> F {
    let t120015 = -t119996 * t510 - t1266 * t33124 - t1442 * t30989 - F::new(4.0) * t22461 * t7472 - F::new(4.0) * t26103 * t7472 - t4026 * t8439 - t119867 - F::new(4.0) * t119869 - F::new(4.0) * t119871 - t119874 + F::new(2.0) * t119875 + t119877 + t120002 - F::new(2.0) * t120003 - F::new(2.0) * t120005 - t120008 - t32679 - t8329;
    t120015
}
