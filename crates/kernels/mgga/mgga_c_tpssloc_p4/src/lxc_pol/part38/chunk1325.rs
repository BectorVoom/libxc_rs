//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1325/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1325<F: Float>(t29895: F, t30165: F, t2331: F, t2585: F, t2: F, t666: F, t29900: F, t30172: F, t110093: F, t110097: F, t1444: F, t1453: F, t2248: F, t2332: F, t2342: F, t2358: F, t26129: F, t29903: F, t29907: F, t29922: F, t29926: F, t30164: F, t30171: F, t30175: F, t659: F, t8128: F, t8137: F, t8138: F) -> F {
    let t110586 = F::new(20.0) / F::new(9.0) * t29895 * t30165;
    let t110601 = t2585 * t2331;
    let t110602 = t2 * t666;
    let t110615 = F::new(20.0) / F::new(27.0) * t29900 * t30172;
    let t110623 = F::new(5.0) / F::new(12.0) * t8128 * t8138 * t1453 * t2248 - t110586 + F::new(5.0) / F::new(2.0) * t29903 * t29907 * t26129 + F::new(5.0) / F::new(18.0) * t8128 * t29926 * t1453 * t2342 - F::new(5.0) / F::new(4.0) * t29903 * t8138 * t1444 * t2332 - F::new(25.0) / F::new(18.0) * t8128 * t29922 * t30164 + F::new(5.0) / F::new(6.0) * t110601 * t8138 * t110602 + F::new(5.0) / F::new(108.0) * t8137 * t110097 * t1444 * t2342 - F::new(5.0) / F::new(18.0) * t30175 * t29926 * t2 * t659 + t110615 + F::new(5.0) / F::new(12.0) * t8128 * t8138 * t1444 * t2358 + F::new(25.0) / F::new(54.0) * t8137 * t110093 * t30171;
    t110623
}
