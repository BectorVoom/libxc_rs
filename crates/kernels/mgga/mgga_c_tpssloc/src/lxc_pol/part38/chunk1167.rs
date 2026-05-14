//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1167/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1167<F: Float>(t29934: F, t510: F, t574: F, t1393: F, t8143: F, t2180: F, t3929: F, t3652: F, t1268: F, t12734: F, t12739: F, t12823: F, t2181: F, t2183: F, t2314: F, t29890: F, t4034: F, t5113: F, t652: F, t8124: F, t8144: F, t8148: F, t8150: F, t9348: F) -> (F, F, F, F, F, F) {
    let t29935 = t510 * t29934;
    let t29944 = t29934 * t574;
    let t29947 = t8143 * t1393;
    let t29956 = t2180 * t3929;
    let t29963 = t3652 * t2180;
    let t29978 = 2.0 * t1268 * t29944 + 4.0 * t1268 * t29947 + 2.0 * t1268 * t29956 - 4.0 * t12734 * t2181 + 4.0 * t12734 * t2183 + 2.0 * t12739 * t2183 - 2.0 * t12823 * t2181 - 2.0 * t2181 * t9348 + 2.0 * t2183 * t9348 - 4.0 * t2314 * t8124 - 4.0 * t2314 * t8144 + 4.0 * t2314 * t8148 + 4.0 * t2314 * t8150 - 4.0 * t29890 * t652 - 2.0 * t29935 * t652 - 2.0 * t29963 * t652 - 4.0 * t4034 * t8124 - 4.0 * t4034 * t8144 + 4.0 * t5113 * t8148 + 4.0 * t5113 * t8150;
    (t29935, t29944, t29947, t29956, t29963, t29978)
}
