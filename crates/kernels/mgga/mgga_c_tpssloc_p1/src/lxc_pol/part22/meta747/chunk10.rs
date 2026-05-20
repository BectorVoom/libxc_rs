//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2498/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2498<F: Float>(t1052: F, t1065: F, t14529: F, t14552: F, t1603: F, t1634: F, t1635: F, t18047: F, t18074: F, t18165: F, t21614: F, t21676: F, t21677: F, t3026: F, t3169: F, t3174: F, t349: F, t388: F, t43604: F, t4665: F, t5920: F, t5944: F, t60971: F, t61061: F, t61621: F, t70938: F, t990: F) -> F {
    let t71049 = F::new(24.0) * t1052 * t1065 * t21676 * t43604 + F::new(6.0) * t1052 * t1634 * t18165 * t3174 + F::new(3.0) * t1603 * t18047 * t388 + t21614 * t388 * t990 + t349 * t388 * t70938 + F::new(6.0) * t14529 * t5920 + F::new(6.0) * t14552 * t5920 - F::new(3.0) * t14552 * t5944 - F::new(6.0) * t1635 * t60971 - F::new(3.0) * t1635 * t61061 - F::new(3.0) * t1635 * t61621 + F::new(6.0) * t18074 * t4665 - F::new(6.0) * t21677 * t3026 - F::new(6.0) * t21677 * t3169;
    t71049
}
