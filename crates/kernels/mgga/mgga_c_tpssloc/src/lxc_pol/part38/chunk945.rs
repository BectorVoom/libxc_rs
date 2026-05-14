//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 945/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk945<F: Float>(t2363: F, t88: F, t1454: F, t2281: F, t4044: F, t626: F, t4068: F, t1453: F, t2332: F, t9365: F, t2331: F, t4067: F, t666: F, t2358: F, t4043: F, t1444: F, t2342: F, t9384: F) -> (F, F, F, F, F, F, F, F) {
    let t12739 = t88 * t2363;
    let t12747 = t2281 * t1454;
    let t12750 = 4.0 / 3.0 * t626 * t4044;
    let t12752 = 2.0 / 3.0 * t626 * t4068;
    let t12754 = t9365 * t1453 * t2332;
    let t12757 = t2331 * t4067;
    let t12758 = t12757 * t666;
    let t12761 = t4043 * t2358;
    let t12771 = t9384 * t1444 * t2342;
    (t12739, t12747, t12750, t12752, t12754, t12758, t12761, t12771)
}
