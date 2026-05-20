//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1187/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1187<F: Float>(t12734: F, t12823: F, t1393: F, t1983: F, t2040: F, t2075: F, t2096: F, t22607: F, t2314: F, t2363: F, t23918: F, t23929: F, t23933: F, t23951: F, t24026: F, t24166: F, t24176: F, t24442: F, t4034: F, t45637: F, t45640: F, t510: F, t574: F, t652: F, t6876: F, t6999: F, t7050: F, t7057: F, t7156: F, t7170: F, t7220: F, t83863: F, t83904: F, t84291: F, t84298: F, t9416: F) -> F {
    let t84322 = -F::new(6.0) * t2314 * t24442 - F::new(6.0) * t4034 * t24442 - F::new(6.0) * t652 * t7156 * t2363 + F::new(3.0) * t1983 * t7170 * t83863 + t83904 * t2096 - F::new(3.0) * t6876 * t23951 - F::new(3.0) * t1983 * t24166 * t6999 + F::new(3.0) * t24026 * t1393 + t84298 * t574 - F::new(12.0) * t2314 * t23933 - t84291 * t510 + F::new(18.0) * t6876 * t24176 - F::new(6.0) * t2314 * t23918 - F::new(6.0) * t45637 * t2040 - F::new(2.0) * t45640 * t2040 - F::new(6.0) * t12823 * t7050 - F::new(2.0) * t652 * t2075 * t9416 - F::new(12.0) * t12734 * t7057 - F::new(12.0) * t2314 * t23929 - F::new(3.0) * t22607 * t7220;
    t84322
}
