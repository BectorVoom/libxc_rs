//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1047/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1047<F: Float>(t12492: F, t12504: F, t1266: F, t12734: F, t12823: F, t1983: F, t2040: F, t2075: F, t2079: F, t2095: F, t22574: F, t22578: F, t22584: F, t2320: F, t2323: F, t23917: F, t23918: F, t23929: F, t23938: F, t24175: F, t26558: F, t3652: F, t39235: F, t3929: F, t4034: F, t510: F, t55183: F, t652: F, t672: F, t7040: F, t7042: F, t7050: F, t7056: F, t7057: F, t7156: F, t7166: F, t7217: F, t83911: F, t84044: F, t84097: F, t9347: F) -> (F,) {
    let t84130 = -6.0 * t12823 * t7057 - 12.0 * t12734 * t7050 - 3.0 * t1983 * t7217 * t22578 + 3.0 * t7166 * t3929 + t2079 * t12492 - t1983 * t2095 * t83911 - 6.0 * t84097 * t672 - t9347 * t2075 - 3.0 * t7040 * t3652 + 18.0 * t22574 * t26558 * t55183 - 6.0 * t652 * t3652 * t7056 - 6.0 * t4034 * t23918 - 6.0 * t652 * t1266 * t23917 - 2.0 * t39235 * t2040 - 2.0 * t652 * t510 * t84044 + 9.0 * t1983 * t24175 * t22584 - 6.0 * t2320 * t7156 - 12.0 * t23938 * t2323 - 6.0 * t7042 * t12504 - 12.0 * t4034 * t23929;
    (t84130,)
}
