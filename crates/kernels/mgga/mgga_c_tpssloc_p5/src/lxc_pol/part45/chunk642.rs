//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 642/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk642<F: Float>(t533: F, t7216: F, t1390: F, t2095: F, t6999: F, t113: F, t1266: F, t1393: F, t1983: F, t2036: F, t2040: F, t2075: F, t2079: F, t2096: F, t2314: F, t4034: F, t510: F, t574: F, t650: F, t652: F, t672: F, t6876: F, t7040: F, t7042: F, t7050: F, t7057: F, t7061: F, t7156: F, t7166: F, t7171: F) -> (F, F, F, F) {
    let t7217 = t533 * t7216;
    let t7218 = t7217 * t1390;
    let t7220 = t2095 * t6999;
    let t7222 = -t113 * t7156 - t1266 * t2036 + t1393 * t2079 + F::new(3.0) * t1983 * t7171 + t1983 * t7218 - t1983 * t7220 - F::new(2.0) * t2040 * t2314 - F::new(2.0) * t2040 * t4034 - t2075 * t650 + t2096 * t6876 - t510 * t7040 + t574 * t7166 - F::new(2.0) * t652 * t7050 - F::new(2.0) * t652 * t7057 - F::new(2.0) * t652 * t7061 - F::new(2.0) * t672 * t7042;
    (t7217, t7218, t7220, t7222)
}
