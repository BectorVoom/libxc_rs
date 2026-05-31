//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1634/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1634<F: Float>(t2035: F, t2319: F, t2095: F, t22578: F, t22584: F, t7170: F, t1266: F, t12734: F, t1393: F, t1983: F, t2036: F, t2040: F, t2079: F, t2314: F, t2323: F, t2364: F, t23909: F, t23918: F, t23929: F, t23933: F, t23938: F, t3652: F, t3929: F, t4034: F, t510: F, t652: F, t672: F, t7040: F, t7042: F, t7050: F, t7057: F, t7061: F, t7166: F, t9348: F) -> (F, F, F, F) {
    let t23941 = t2035 * t2319;
    let t23951 = t2095 * t22578;
    let t23953 = t7170 * t22584;
    let t23956 = -F::cast_from(2.0_f64) * t1266 * t7040 - F::cast_from(4.0_f64) * t12734 * t2040 + F::cast_from(2.0_f64) * t1393 * t7166 - t1983 * t23951 + F::cast_from(3.0_f64) * t1983 * t23953 - t2036 * t3652 - F::cast_from(2.0_f64) * t2040 * t9348 + t2079 * t3929 - F::cast_from(4.0_f64) * t2314 * t7050 - F::cast_from(4.0_f64) * t2314 * t7061 - F::cast_from(4.0_f64) * t2323 * t7042 - F::cast_from(2.0_f64) * t2364 * t7042 - F::cast_from(2.0_f64) * t23909 * t652 - F::cast_from(2.0_f64) * t23918 * t652 - F::cast_from(4.0_f64) * t23929 * t652 - F::cast_from(4.0_f64) * t23933 * t652 - F::cast_from(4.0_f64) * t23938 * t672 - F::cast_from(2.0_f64) * t23941 * t510 - F::cast_from(4.0_f64) * t4034 * t7057;
    (t23941, t23951, t23953, t23956)
}
