//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2469/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2469<F: Float>(t14527: F, t225: F, t14534: F, t10165: F, t10166: F, t10167: F, t10170: F, t1052: F, t1066: F, t13743: F, t14549: F, t14555: F, t14659: F, t1634: F, t1635: F, t3026: F, t3169: F, t3175: F, t3207: F, t381: F, t388: F, t43599: F, t43604: F, t4660: F, t4665: F, t4693: F, t48427: F) -> F {
    let t50690 = t14527 * t225;
    let t50703 = t14534 * t225;
    let t50712 = -F::new(18.0) * t10165 * t1052 * t3175 * t4693 + F::new(24.0) * t10166 * t1052 * t1634 * t43604 + t381 * t388 * t48427 - F::new(6.0) * t10167 * t4660 + F::new(6.0) * t10170 * t4665 - F::new(3.0) * t1066 * t50690 - F::new(3.0) * t1066 * t50703 + F::new(12.0) * t13743 * t3026 + F::new(12.0) * t13743 * t3169 + F::new(6.0) * t14549 * t3169 - F::new(3.0) * t14555 * t3207 - F::new(3.0) * t14659 * t3026 - F::new(3.0) * t1635 * t43599;
    t50712
}
