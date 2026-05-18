//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1179/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1179<F: Float>(t109: F, t81437: F, t81440: F, t81443: F, t81445: F, t81447: F, t81450: F, t81452: F, t112: F, t24447: F, t111: F, t24007: F, t12492: F, t12504: F, t1266: F, t12734: F, t12823: F, t1983: F, t2040: F, t2075: F, t2079: F, t2095: F, t22574: F, t22578: F, t22584: F, t2320: F, t2323: F, t23917: F, t23918: F, t23929: F, t23938: F, t24175: F, t26558: F, t3652: F, t39235: F, t3929: F, t4034: F, t510: F, t55183: F, t652: F, t672: F, t7040: F, t7042: F, t7050: F, t7056: F, t7057: F, t7156: F, t7166: F, t7217: F, t83911: F, t9347: F) -> (F, F, F, F) {
    let t110 = F::new(1.0) < t109;
    let t84036 = F::new(308.0) / F::new(27.0) * t81437;
    let t84044 = piecewise3::<f64>(t110, F::new(0.0), -t84036 - F::new(22.0) / F::new(3.0) * t81440 - F::new(4.0) * t81443 + F::new(2.0) * t81445 - F::new(3.0) / F::new(2.0) * t81447 + F::new(3.0) / F::new(2.0) * t81450 - t81452 / F::new(4.0));
    let t84078 = t24447 * t112;
    let t84097 = t24007 * t111;
    let t84130 = -F::new(6.0) * t12823 * t7057 - F::new(12.0) * t12734 * t7050 - F::new(3.0) * t1983 * t7217 * t22578 + F::new(3.0) * t7166 * t3929 + t2079 * t12492 - t1983 * t2095 * t83911 - F::new(6.0) * t84097 * t672 - t9347 * t2075 - F::new(3.0) * t7040 * t3652 + F::new(18.0) * t22574 * t26558 * t55183 - F::new(6.0) * t652 * t3652 * t7056 - F::new(6.0) * t4034 * t23918 - F::new(6.0) * t652 * t1266 * t23917 - F::new(2.0) * t39235 * t2040 - F::new(2.0) * t652 * t510 * t84044 + F::new(9.0) * t1983 * t24175 * t22584 - F::new(6.0) * t2320 * t7156 - F::new(12.0) * t23938 * t2323 - F::new(6.0) * t7042 * t12504 - F::new(12.0) * t4034 * t23929;
    (t84044, t84078, t84097, t84130)
}
