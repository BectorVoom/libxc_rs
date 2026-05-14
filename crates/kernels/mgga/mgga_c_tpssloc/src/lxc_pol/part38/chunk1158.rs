//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1158/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1158<F: Float>(t15903: F, t15929: F, t15981: F, t16501: F, t113: F, t1266: F, t1271: F, t12724: F, t12728: F, t12835: F, t12841: F, t1393: F, t15857: F, t1774: F, t1778: F, t2312: F, t2314: F, t2320: F, t3929: F, t4026: F, t4037: F, t4077: F, t510: F, t5107: F, t5118: F, t513: F, t5361: F, t650: F, t652: F) -> (F, F) {
    let t16503 = t15903 + t15929 + t15981 + t16501;
    let t16505 = -t113 * t15857 - 2.0 * t1266 * t4026 + 2.0 * t1271 * t5361 - t12724 * t510 - 2.0 * t12728 * t510 - 2.0 * t12835 * t652 - 2.0 * t12841 * t652 + 2.0 * t1393 * t5118 + t16503 * t513 - t1774 * t2312 - 2.0 * t1774 * t2320 + t1778 * t3929 - 4.0 * t2314 * t4037 - 4.0 * t2314 * t4077 - 2.0 * t5107 * t650;
    (t16503, t16505)
}
