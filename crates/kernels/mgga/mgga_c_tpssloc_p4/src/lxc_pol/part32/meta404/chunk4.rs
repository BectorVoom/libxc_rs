//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1544/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1544<F: Float>(t17614: F, t17640: F, t17684: F, t17725: F, t17900: F, t17967: F, t18007: F, t18044: F, t349: F, t1052: F, t1066: F, t17575: F, t17579: F, t17583: F, t17588: F, t3026: F, t3169: F, t388: F, t4557: F, t4660: F, t4665: F, t4694: F, t5920: F, t5944: F) -> (F, F) {
    let t18047 = t17614 + t17640 + t17684 + t17725 + t17900 + t17967 + t18007 + t18044;
    let t18048 = t349 * t18047;
    let t18050 = F::new(4.0) * t1052 * t17583 - t1066 * t17575 - F::new(2.0) * t1066 * t17588 + F::new(2.0) * t17579 * t388 + t18048 * t388 + F::new(2.0) * t3026 * t5920 - t3026 * t5944 + F::new(2.0) * t3169 * t5920 - t3169 * t5944 + F::new(4.0) * t4557 * t4665 - F::new(2.0) * t4557 * t4694 - F::new(2.0) * t4660 * t4694;
    (t18047, t18050)
}
