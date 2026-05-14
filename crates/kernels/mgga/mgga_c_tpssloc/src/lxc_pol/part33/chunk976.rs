//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 976/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk976<F: Float>(t1751: F, t6218: F, t1246: F, t11881: F, t11888: F, t11914: F, t1244: F, t15027: F, t15245: F, t1729: F, t1756: F, t1758: F, t19201: F, t22114: F, t22341: F, t22349: F, t22355: F, t22358: F, t22361: F, t22365: F, t22369: F, t22372: F, t22375: F, t22387: F, t3610: F, t3624: F, t470: F, t494: F, t5064: F, t6168: F, t6253: F, t6257: F, t6261: F, t6263: F, t6265: F) -> (F,) {
    let t22389 = t1751 * t6218;
    let t22390 = t22389 * t1246;
    let t22393 = 3.0 * t1244 * t22341 + 3.0 * t5064 * t6261 + 6.0 * t5064 * t6257 + t11914 * t22349 + 3.0 * t19201 * t1756 - 3.0 * t3624 * t22355 + 6.0 * t11881 * t22358 - 6.0 * t11888 * t22361 + 6.0 * t3610 * t22365 + 6.0 * t3610 * t22369 - 3.0 * t3624 * t22372 + t470 * t22375 + 3.0 * t1729 * t6265 + 6.0 * t15027 * t6253 - 3.0 * t15245 * t6263 + t22114 * t494 + 3.0 * t6168 * t1758 + t1244 * t22387 + 3.0 * t1244 * t22390;
    (t22393,)
}
